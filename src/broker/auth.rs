use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use super::client::BrokerClient;
use crate::error::ClientError;

pub struct AccessToken;

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
}

#[derive(Serialize)]
struct TokenRequestBody<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    grant_type: &'a str,
}

impl AccessToken {
    pub async fn issue(
        client: &BrokerClient,
        client_id: &str,
        client_secret: &str,
    ) -> Result<TokenResponse, ClientError>
    {
        let path = "/oauth2/token";
        let body = TokenRequestBody {
            grant_type: "client_credentials",
            client_id: client_id,
            client_secret: client_secret,
        };
        let header = HeaderMap::new();
        let res_body = 
            client.post_urlencoded(path, header, &body).await?;
        
        Ok(res_body)
    }

}

