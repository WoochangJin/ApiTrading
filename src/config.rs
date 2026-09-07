pub struct Config {
    pub appkey: String,
    pub secretkey: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            appkey: std::env::var("APPKEY").expect("APPKEY not set"),
            secretkey: std::env::var("SECRETKEY").expect("SECRETKEY not set"),
        }
    }
}