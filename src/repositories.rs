use sqlx::SqlitePool;
use crate::models::PriceRecord;

pub async fn get_record(pool: &SqlitePool, ticker: &str, date: &str) -> Result<Option<PriceRecord>, sqlx::Error> {
    sqlx::query_as::<_, PriceRecord>(
        "SELECT * FROM price_records WHERE ticker = ? AND date = ?"
    )
    .bind(ticker)
    .bind(date)
    .fetch_optional(pool)
    .await
}

pub async fn get_latest_record(pool: &SqlitePool, ticker: &str) -> Result<Option<PriceRecord>, sqlx::Error> {
    sqlx::query_as::<_, PriceRecord>(
        "SELECT * FROM price_records WHERE ticker = ? ORDER BY date DESC LIMIT 1"
    )
    .bind(ticker)
    .fetch_optional(pool)
    .await
}

pub async fn insert_record(pool: &SqlitePool, r: &PriceRecord) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO price_records
         (cycle_id, ticker, date, cycle_principal, cycle_start_date, buy_price, buy_qty, sell_price, sell_qty)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(r.cycle_id)
    .bind(&r.ticker)
    .bind(&r.date)
    .bind(r.cycle_principal)
    .bind(&r.cycle_start_date)
    .bind(r.buy_price)
    .bind(r.buy_qty)
    .bind(r.sell_price)
    .bind(r.sell_qty)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_fill_status(
    pool: &SqlitePool,
    ticker: &str,
    date: &str,
    buy_filled: bool,
    sell_filled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE price_records SET buy_filled = ?, sell_filled = ? WHERE ticker = ? AND date = ?"
    )
    .bind(buy_filled)
    .bind(sell_filled)
    .bind(ticker)
    .bind(date)
    .execute(pool)
    .await?;
    Ok(())
}