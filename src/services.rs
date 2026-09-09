use chrono::Local;
use crate::state::AppState;
use crate::repositories;
use crate::models::PriceRecord;
use crate::broker::account::BuyingPower;
use crate::broker::order::{OrderRequest, OrderStatus, Side};
use crate::price::caller::PriceRequest;
use crate::error::ClientError;
use crate::notification;

const TICKER: &str = "SOXL";
const CURRENCY: &str = "USD";

pub async fn run_daily_cycle(state: &AppState) -> Result<(), ClientError> {
    let today = Local::now().date_naive().to_string();

    // 0. 중복 실행 방지
    if repositories::get_record(&state.db, TICKER, &today).await?.is_some() {
        tracing::info!("{TICKER} already processed for {today}, skipping");
        return Ok(());
    }

    let prev = repositories::get_latest_record(&state.db, TICKER).await?;

    // 1. 어제 체결 확인
    let mut buy_filled_yesterday = false;
    let mut sell_filled_yesterday = false;
    if let Some(prev) = &prev {
        let history = state.with_token(|token| {
            OrderRequest::get_order_history(
                &state.broker_client, token, state.account_seq,
                "CLOSED", Some(TICKER), Some(&prev.date), Some(&prev.date),
            )
        }).await?;

        let buy_filled = history.orders.iter()
            .any(|o| matches!(o.side, Side::Buy) && matches!(o.status, OrderStatus::Filled));
        let sell_filled = history.orders.iter()
            .any(|o| matches!(o.side, Side::Sell) && matches!(o.status, OrderStatus::Filled));

        repositories::update_fill_status(&state.db, TICKER, &prev.date, buy_filled, sell_filled).await?;
        buy_filled_yesterday = buy_filled;
        sell_filled_yesterday = sell_filled;
    }

    // 2. 오늘 사이클 정보 결정
    let (cycle_id, principal, start_date) = match &prev {
        None => (1, state.initial_principal, state.initial_start_date.clone()),
        Some(p) if sell_filled_yesterday => {
            // 사이클이 끝났으므로 실제 매수 가능 금액을 다음 사이클 원금으로 삼는다
            let buying_power = state.with_token(|token| {
                BuyingPower::get_buying_power(
                    &state.broker_client, token, state.account_seq, CURRENCY,
                )
            }).await?;

            let next_principal = match buying_power.cash_buying_power.parse::<f64>() {
                Ok(v) if v > 0.0 => {
                    tracing::info!(
                        "cycle {} closed: principal {} -> {v}",
                        p.cycle_id, p.cycle_principal,
                    );
                    v
                }
                other => {
                    // 파싱 실패하거나 0 이면 원금을 잃지 않도록 기존 값을 유지한다
                    tracing::warn!(
                        "cycle {} closed but buying power was unusable ({:?} -> {other:?}); keeping principal {}",
                        p.cycle_id, buying_power.cash_buying_power, p.cycle_principal,
                    );
                    p.cycle_principal
                }
            };

            (p.cycle_id + 1, next_principal, today.clone())
        }
        Some(p) => (p.cycle_id, p.cycle_principal, p.cycle_start_date.clone()),
    };

    // 3. 오늘 가격 조회
    let price = PriceRequest::get_price(&state.price_client, TICKER, principal, &start_date).await?;

    // 4. 주문
    if price.buy_qty > 0 {
        let qty_str = price.buy_qty.to_string();
        let price_str = price.buy_price.to_string();
        let client_order_id = format!("{TICKER}-{today}-buy");
        state.with_token(|token| {
            OrderRequest::loc_order(
                state.account_seq, token, &state.broker_client,
                &qty_str, "BUY", &price_str, Some(&client_order_id),
            )
        }).await?;
    }

    if price.sell_qty > 0 {
        let qty_str = price.sell_qty.to_string();
        let price_str = price.sell_price.to_string();
        let client_order_id = format!("{TICKER}-{today}-sell");
        state.with_token(|token| {
            OrderRequest::loc_order(
                state.account_seq, token, &state.broker_client,
                &qty_str, "SELL", &price_str, Some(&client_order_id),
            )
        }).await?;
    }

    // 5. 오늘 레코드 저장
    let record = PriceRecord {
        id: 0,
        cycle_id,
        ticker: TICKER.to_string(),
        date: today.clone(),
        cycle_principal: principal,
        cycle_start_date: start_date,
        buy_price: Some(price.buy_price),
        buy_qty: price.buy_qty as i64,
        buy_filled: None,
        sell_price: Some(price.sell_price),
        sell_qty: price.sell_qty as i64,
        sell_filled: None,
    };
    repositories::insert_record(&state.db, &record).await?;

    // 6. 디스코드 알림
    let message = format!(
        "**{TICKER} 데일리 사이클 ({today})**\n\
         어제 매수체결: {buy_filled_yesterday}\n\
         어제 매도체결: {sell_filled_yesterday}\n\
         오늘 매수: {}주 @ ${}\n\
         오늘 매도: {}주 @ ${}",
        price.buy_qty, price.buy_price,
        price.sell_qty, price.sell_price,
    );
    notification::send_discord(&state.discord_client, &message).await;

    Ok(())
}