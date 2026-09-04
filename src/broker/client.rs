/*
Rest Client Base
*/

use reqwest::{Client, Method, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned};
use crate::error::ClientError;

pub struct BrokerClient {
    http: Client,
    base_url: String,
}

impl BrokerClient {
    pub fn new(base_url: String) -> Self {
        BrokerClient {
            http: Client::new(),
            base_url,
        }
    }

    pub async fn get<B: Serialize, T: DeserializeOwned>(
        &self, 
        path: &str,
        header: HeaderMap,
        query: &B,
    ) -> Result<T, ClientError> {
        let res = self.http
            .get(format!("{}{}", self.base_url, path))
            .headers(header)
            .query(query)
            .send()
            .await?
            .error_for_status()?;
        Ok(res.json::<T>().await?)
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        header: HeaderMap,
        body: &B,
    ) -> Result<T, ClientError> {
        let res = self.http
            .post(format!("{}{}", self.base_url, path))
            .headers(header)
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        Ok(res.json::<T>().await?)
    }
} 