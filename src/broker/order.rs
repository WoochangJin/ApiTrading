use super::client::BrokerClient;
use crate::error::ClientError;
use serde::{Serialize, Deserialize};
use reqwest::header::HeaderMap;

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

impl OrderRequest {
    pub async fn loc_order(
        x_tossinvest_account: i64,
        token: &str,
        client: &BrokerClient,
        quantity: &str,
        side: &str,
        price: &str,
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
        };
        let mut header = HeaderMap::new();
        header.insert("authorization", format!("Bearer {token}").parse().unwrap());
        header.insert("X-Tossinvest-Account", x_tossinvest_account.into());
        let envlope:OrderResultEnvelope = client.post(path, header, &body).await?;

        

        Ok(envlope.result)
    }
}