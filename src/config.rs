pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub toss_api_url: String,
    pub price_server_url: String,
    pub price_server_local_url: String,
    pub database_url: String,
    pub account_seq: i64,
    pub initial_principal: f64,
    pub initial_start_date: String,
    pub discord_webhook_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            client_id: std::env::var("CLIENT_ID").expect("CLIENT_ID not set"),
            client_secret: std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set"),
            toss_api_url: std::env::var("TOSS_API_URL").expect("TOSS_API_URL not set"),
            price_server_local_url: std::env::var("PRICE_SERVER_LOCAL_URL").expect("PRICE_SERVER_LOCAL_URL not set"),
            price_server_url: std::env::var("PRICE_SERVER_URL").expect("PRICE_SERVER_URL not set"),
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            account_seq: std::env::var("ACCOUNT_SEQ").expect("ACCOUNT_SEQ not set").parse().expect("ACCOUNT_SEQ must be a valid integer"),
            initial_principal: std::env::var("INITIAL_PRINCIPAL")
                .expect("INITIAL_PRINCIPAL not set")
                .parse()
                .expect("INITIAL_PRINCIPAL must be a valid number"),
            initial_start_date: std::env::var("INITIAL_START_DATE")
                .expect("INITIAL_START_DATE not set"),
            discord_webhook_url: std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL not set"),
        }
    }
}