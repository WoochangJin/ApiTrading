use serde::Deserialize;
use super::client::BrokerClient;
use crate::{broker::client, error::ClientError};

struct Accounts;

#[derive(Deserialize)]
struct AccountSResponse {
    result: AccountsResponseBody,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsResponseBody {
    pub account_no: String,
    pub account_seq: i32,
    pub account_type: String,
}

impl Accounts {
    pub async fn get_accounts(
        client: &BrokerClient,
        token: &str,
    ) -> Result<AccountsResponseBody, ClientError> {
        let path = "/api/v1/accounts";
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("Authorization", format!("Bearer {token}").parse().unwrap());
        let res: AccountSResponse = client.get(path, header, &()).await?;
        let res_body = res.result;
        Ok(res_body)
    }
}