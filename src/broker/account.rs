use serde::Deserialize;
use super::client::BrokerClient;
use crate::{broker::client, error::ClientError};

pub struct Accounts;

#[derive(Deserialize)]
struct AccountSResponse {
    result: Vec<AccountsResponseBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsResponseBody {
    pub account_no: Option<String>,
    pub account_seq: Option<i32>,
    pub account_type: Option<String>,
}

impl Accounts {
    pub async fn get_accounts(
        client: &BrokerClient,
        token: &str,
    ) -> Result<Vec<AccountsResponseBody>, ClientError> {
        let path = "/api/v1/accounts";
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("Authorization", format!("Bearer {token}").parse().unwrap());
        let res: AccountSResponse = client.get(path, header, &()).await?;
        let res_body = res.result;
        Ok(res_body)
    }
}