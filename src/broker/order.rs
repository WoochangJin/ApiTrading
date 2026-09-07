use super::client::BrokerClient;
use crate::error::ClientError;
use serde::{Serialize, Deserialize};

pub struct OrderRequest;

struct BuyRequestBody<'a> {
    pub ticker: &'a str,
    pub principal: i32,
    pub start_date: &'a str,
}