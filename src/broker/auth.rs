use axum::{Router, body, extract::path, http::HeaderMap, routing::get};
use serde::{Deserialize, Serialize};
use super::client::BrokerClient;
use crate::{broker::client, error::ClientError};

pub struct AccessToken{
    base_url: String,
}

#[derive(Deserialize)]
pub struct AccessTokenResponse {
    pub token: String,
    pub expires_dt: String,
    pub token_type: String,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    appkey: &'a str,
    secretkey: &'a str,
}

#[derive(Deserialize)]
struct RevokeResponse {
    return_code: i32,
    return_msg: String,
}

#[derive(Serialize)]
struct RevokeRequest<'a> {
    token: &'a str,
    appkey: &'a str,
    secretkey: &'a str,
}

impl AccessToken {
    pub async fn issue(
        client: &BrokerClient,
        appkey: &str,
        secretkey: &str,
    ) -> Result<AccessTokenResponse, ClientError>
    {
        let path = "/oauth2/token";
        let body = TokenRequest {
            grant_type: "client_credentials",
            appkey,
            secretkey,
        };
        let header = HeaderMap::new();
        let res_body = 
            client.post(path, header, &body).await?;
        
        Ok(res_body)
    }

    pub async fn revoke(
        client: &BrokerClient,
        appkey: &str,
        secretkey: &str,
        token: &str
    ) -> Result<(), ClientError>
    {
        let path = "/oauth2/revoke";
        let header = HeaderMap::new();
        let body = RevokeRequest {
            token,
            appkey,
            secretkey,
        };
        let res :RevokeResponse = client.post(path, header, &body).await?;

        if res.return_code != 0 {
            return Err(ClientError::Api {
                code: res.return_code.to_string(),
                message: res.return_msg,
            });
        }
        Ok(())
    }
}

