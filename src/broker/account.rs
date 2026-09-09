use serde::Deserialize;
use super::client::BrokerClient;
use crate::error::ClientError;

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
        token: String,
    ) -> Result<Vec<AccountsResponseBody>, ClientError> {
        let path = "/api/v1/accounts";
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("Authorization", format!("Bearer {token}").parse().unwrap());
        let res: AccountSResponse = client.get(path, header, &()).await?;
        let res_body = res.result;
        Ok(res_body)
    }
}
pub struct BuyingPower;

#[derive(Deserialize)]
struct BuyingPowerEnvelope {
    result: BuyingPowerResponse,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyingPowerResponse {
    pub currency: String,
    /// 현금 기반 매수 가능 금액 (미수 미발생 기준). USD 는 소수점 포함 문자열
    pub cash_buying_power: String,
}

#[derive(serde::Serialize)]
struct BuyingPowerQuery<'a> {
    currency: &'a str,
}

impl BuyingPower {
    pub async fn get_buying_power(
        client: &BrokerClient,
        token: String,
        x_tossinvest_account: i64,
        currency: &str,
    ) -> Result<BuyingPowerResponse, ClientError> {
        let path = "/api/v1/buying-power";
        let query = BuyingPowerQuery { currency };

        let mut header = reqwest::header::HeaderMap::new();
        header.insert("authorization", format!("Bearer {token}").parse().unwrap());
        header.insert("X-Tossinvest-Account", x_tossinvest_account.into());

        let envelope: BuyingPowerEnvelope = client.get(path, header, &query).await?;
        Ok(envelope.result)
    }
}
