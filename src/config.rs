pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub toss_api_url: String,
    pub price_server_url: String,
    pub price_server_local_url: String,
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
        }
    }
}