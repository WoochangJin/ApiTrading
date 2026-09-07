mod config;
mod state;
mod error;
mod broker;
mod dto;
mod handlers;
mod models;
mod notification;
mod repositories;
mod risk;
mod routes;
mod scheduler;
mod services;
mod strategy;
mod ws;
mod price;

use broker::client::BrokerClient;
use broker::auth::AccessToken;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = config::Config::from_env();
    let broker_client = BrokerClient::new("https://openapi.tossinvest.com".to_string());
    let order = broker::order::OrderRequest::loc_order(
        2,
        "eyJraWQiOiIyMzMyNzY5MmJmMDQwOTM0YjBmMDVjODA2MWM4MjE5ODJjMDlkZDU2ZDcwNGQxZTNhNjE4OGZiNWFhMDY2MWQ0IiwiYWxnIjoiUlMyNTYifQ.eyJzdWIiOiJ0c2NrX2xpdmVfWjI2eWNSbHVKNW9qekg5TzhjVlp2QyIsImF1ZCI6InRzY2tfbGl2ZV9aMjZ5Y1JsdUo1b2p6SDlPOGNWWnZDIiwibmJmIjoxNzg4Nzg5NTMxLCJjbGllbnRJZCI6InRzY2tfbGl2ZV9aMjZ5Y1JsdUo1b2p6SDlPOGNWWnZDIiwidGllciI6IkJBU0lDIiwiaXNzIjoiaHR0cHM6Ly9vcGVuYXBpLnRvc3NpbnZlc3QuY29tIiwiZXhwIjoxNzg4ODc1OTMxLCJpYXQiOjE3ODg3ODk1MzEsImp0aSI6IjQ4ODBhNTQwLTdhZTctNDkzZi1hMTVhLTg1NTNiYjAwYjdjMiJ9.oL01AM5WqwFP8YzocDFFlIZtAlz7dMu-lCRxW9HcFPBkCkPtYEicYJFzpDahmv4nEKbd4CJ61Nm9eZSLd6tRNuUNL5cTOAyJLlQxlCExlcO7mkLC1ls9S8e1CgQpVWMMC1XxYbcc7WBVeUzcttasf-h85MXOqHSPRgtiwAegRbx6HCt1EVfX4C6abQkwUpdU0J1lCGXbLa5bynXnwg5jzNVLoHrL1cqwPwxFHnbzadJpZhv1a-G8Jn4KPwYg-i9ZVD909BmDkkZX-6sBrmv6aw0FLqZluYPeabl0pBhTVTSukSukzvmPSrzC7PyDAHvzp03M5PUmWYTmP_zq51X6FA",
        &broker_client,
        "10",
        "BUY",
        "10",
    ).await;

    println!("Order response: {:?}", order);
}
