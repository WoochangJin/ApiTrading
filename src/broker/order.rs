use super::client::BrokerClient;
use crate::error::ClientError;
use serde::{Serialize, Deserialize};
use reqwest::header::HeaderMap;
use sqlx::query;

pub struct OrderRequest;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderRequestBody<'a> {
    order_type: &'a str,
    quantity: &'a str,
    side: &'a str,
    symbol: &'a str,
    confirm_high_value_order: bool,
    price: &'a str,
    time_in_force: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_order_id: Option<&'a str>,
}

#[derive(Deserialize)]
struct OrderResultEnvelope {
    result: OrderResponse,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub order_id: String,
    pub client_order_id: String,
}

#[derive(Deserialize)]
struct OrderHistoryEnvelope {
    result: OrderHistoryResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryResult {
    pub orders: Vec<OrderHistoryItem>,
    pub next_cursor: Option<String>,
    pub has_next: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryItem {
    pub order_id: String,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub status: OrderStatus,
    pub price: String,
    pub quantity: String,
    pub order_amount: Option<String>,
    pub currency: String,
    pub ordered_at: String,
    pub canceled_at: Option<String>,
    pub execution: OrderExecution,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderExecution {
    pub filled_quantity: String,
    pub average_filled_price: Option<String>,
    pub filled_amount: Option<String>,
    pub commission: Option<String>,
    pub tax: Option<String>,
    pub filled_at: Option<String>,
    pub settlement_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    PartialFilled,
    Filled,
    Canceled,
    Rejected,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub enum Side {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Debug, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
}

#[derive(Debug, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "CLS")]
    Cls,
    #[serde(rename = "OPG")]
    Opg,
}

#[derive(Serialize)]
struct OrderHistoryQuery<'a> {
    status: &'a str,   // "OPEN" | "CLOSED"
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<&'a str>,   // "YYYY-MM-DD", KST 기준
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl OrderRequest {
    pub async fn loc_order(
        x_tossinvest_account: i64,
        token: &str,
        client: &BrokerClient,
        quantity: &str,
        side: &str,
        price: &str,
        client_order_id: Option<&str>,
    ) -> Result<OrderResponse, ClientError> {
        let path = "/api/v1/orders";
        let body = OrderRequestBody {
            order_type: "LIMIT",
            quantity,
            side,
            symbol: "SOXL",
            confirm_high_value_order: false,
            price,
            time_in_force: "CLS",
            client_order_id,
        };
        let mut header = HeaderMap::new();
        header.insert("authorization", format!("Bearer {token}").parse().unwrap());
        header.insert("X-Tossinvest-Account", x_tossinvest_account.into());
        let envelope:OrderResultEnvelope = client.post(path, header, &body).await?;

        

        Ok(envelope.result)
    }

    pub async fn get_order_history(
        client: &BrokerClient,
        token: &str,
        x_tossinvest_account: i64,
        status: &str,
        symbol: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<OrderHistoryResult, ClientError> {
        let path = "/api/v1/orders";
        let query = OrderHistoryQuery { status, symbol, from, to, cursor: None, limit: None };

        let mut header = HeaderMap::new();
        header.insert("authorization", format!("Bearer {token}").parse().unwrap());
        header.insert("X-Tossinvest-Account", x_tossinvest_account.into());

        let envelope: OrderHistoryEnvelope = client.get(path, header, &query).await?;
        Ok(envelope.result)
    }
}