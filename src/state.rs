use sqlx::SqlitePool;
use tokio::sync::Mutex;
use std::future::Future;
use crate::broker::client::BrokerClient;
use crate::error::ClientError;
use crate::broker::auth::AccessToken;
use crate::config::Config;

pub struct AppState {
    pub db: SqlitePool,
    pub broker_client: BrokerClient,
    pub price_client: BrokerClient,
    pub token: Mutex<Option<String>>,   // 캐시된 토큰
    pub client_id: String,
    pub client_secret: String,
}

impl AppState {
    pub async fn new(config: &Config, db: SqlitePool) -> Self {
        Self {
            db,
            broker_client: BrokerClient::new(config.toss_api_url.clone()),
            price_client: BrokerClient::new(config.price_server_url.clone()),
            token: Mutex::new(None),
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
        }
    }

    async fn get_or_issue_token(&self) -> Result<String, ClientError> {
        let cached = self.token.lock().await.clone();
        match cached {
            Some(t) => Ok(t),
            None => self.refresh_token().await,
        }
    }

    async fn refresh_token(&self) -> Result<String, ClientError> {
        let res = AccessToken::issue(&self.broker_client, &self.client_id, &self.client_secret).await?;
        let mut guard = self.token.lock().await;
        *guard = Some(res.access_token.clone());
        Ok(res.access_token)
    }


    pub async fn with_token<F, Fut, T>(&self, f: F) -> Result<T, ClientError>
    where
        F: Fn(String) -> Fut,
        Fut: Future<Output = Result<T, ClientError>>,
    {
        let token = self.get_or_issue_token().await?;
        match f(token.clone()).await {
            Err(ClientError::Api { code, .. }) if code == "invalid-token" => {
                let new_token = self.refresh_token().await?;
                f(new_token).await
            }
            other => other,
        }
    }
}