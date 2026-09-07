use crate::broker::client::BrokerClient;
use crate::error::ClientError;
use reqwest::header::HeaderMap;
use serde::{Serialize, Deserialize};

pub struct PriceRequest;

#[derive(Serialize)]
struct PriceRequestBody<'a> {
    pub ticker: &'a str,
    pub principal: i32,
    pub start_date: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct PriceResponse {
    pub buy_price: f64,
    pub buy_qty: i32,
    pub sell_price: f64,
    pub sell_qty: i32,
}

impl PriceRequest {
    pub async fn get_price(
        client: &BrokerClient,
        ticker: &str,
        principal: i32,
        start_date: &str,
    ) -> Result<PriceResponse, ClientError> {
        let path = "/order";
        let body = PriceRequestBody {
            ticker: ticker,
            principal,
            start_date: start_date,
        };
        let header = HeaderMap::new();
        let res_body = 
            client.post(path, header, &body).await?;
        
        Ok(res_body)
    }
}