/*
Rest Client Base
*/

use axum::extract::path;
use reqwest::{Client, Method, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use crate::error::ClientError;

pub struct BrokerClient {
    http: Client,
    base_url: String,
}

#[derive(Deserialize)]
struct ApiErrorEnvelope {
    error: ApiErrorBody,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiErrorBody {
    code: String,
    message: String,
    data: Option<serde_json::Value>,
    request_id: Option<String>,
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
            .await?;
        if !res.status().is_success() {
            let err_body: ApiErrorEnvelope = res.json().await
                .unwrap_or_else(|_| ApiErrorEnvelope {
                    error: ApiErrorBody { 
                        code: "unknown".into(), 
                        message: "failed to parse error body".into(),
                        data: None,
                        request_id: None,
                    }
            });
            return Err(ClientError::Api { 
                code: err_body.error.code, 
                message: err_body.error.message,
                data: err_body.error.data,
                request_id: err_body.error.request_id,
            });
        }
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
            .await?;

        if !res.status().is_success() {
            let err_body: ApiErrorEnvelope = res.json().await
                .unwrap_or_else(|_| ApiErrorEnvelope {
                    error: ApiErrorBody { 
                        code: "unknown".into(), 
                        message: "failed to parse error body".into(),
                        data: None,
                        request_id: None,
                    }
            });
            return Err(ClientError::Api { 
                code: err_body.error.code, 
                message: err_body.error.message,
                data: err_body.error.data,
                request_id: err_body.error.request_id,
            });
        }
        Ok(res.json::<T>().await?)
    }

    pub async fn post_empty<B: Serialize>(
        &self,
        path: &str,
        header: HeaderMap,
        body: &B,
    ) -> Result<(), ClientError> {
        self.http
            .post(format!("{}{}", self.base_url, path))
            .headers(header)
            .json(body)
            .send()
            .await?
            .error_for_status()?;   // 상태코드만 확인, .json() 호출 안 함

        Ok(())
    }

    pub async fn post_urlencoded<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        header: HeaderMap,
        body: &B,
    ) -> Result<T, ClientError> {
        let res = self.http
            .post(format!("{}{}", self.base_url, path))
            .headers(header)
            .form(body)
            .send()
            .await?;

        if !res.status().is_success() {
            let err_body: ApiErrorEnvelope = res.json().await
                .unwrap_or_else(|_| ApiErrorEnvelope {
                    error: ApiErrorBody { 
                        code: "unknown".into(), 
                        message: "failed to parse error body".into(), 
                        data: None, 
                        request_id: None 
                    }
                });
                return Err(ClientError::Api { 
                    code: err_body.error.code, 
                    message: err_body.error.message, 
                    data: err_body.error.data,
                    request_id: err_body.error.request_id,
                });
        }

    Ok(res.json::<T>().await?)
    }
} 