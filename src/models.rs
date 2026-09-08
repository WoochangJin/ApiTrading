use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PriceRecord {
    pub id: i64,
    pub cycle_id: i64,
    pub ticker: String,
    pub date: String,
    pub cycle_principal: f64,
    pub cycle_start_date: String,
    pub buy_price: Option<f64>,
    pub buy_qty: i64,
    pub buy_filled: Option<bool>,
    pub sell_price: Option<f64>,
    pub sell_qty: i64,
    pub sell_filled: Option<bool>,
}