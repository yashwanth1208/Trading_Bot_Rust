use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]

struct StockPrice{
    stockPrice: String,
}

pub async fn run_trading_bot(){
    let node_api_url = env::var("NODE_API_URL").expect("NODE_API_URL must be set");
    
    let mut balance: f64 = 10000.0;  // Initializing and mutating(Have to be updated) balance in INR
    let mut stock_holding: f64 = 0.0;

    loop {
        let response: StockPrice = reqwest::get(&format!("{}/stock-price", node_api_url))
            .await.expect("Failed to fetch stock price")
            .json().await.expect("Invalid JSON response");
        
        let stock_price: f64 = response.stockPrice.parse().expect("Failed to parse stock price");

        // trading logic: Buy if drops by 2%, sell if raises by 3%
        if stock_holding == 0.0 && stock_price <= 98.0 {
            stock_holding = balance / stock_price;
            balance = 0.0;
            println!("Bought stocks at price: ${}", stock_price);
        } else if stock_holding > 0.0 && stock_price >= 103.0 {
            balance = stock_holding * stock_price;
            stock_holding = 0.0;
            println!("Sold stocks at price: ${}", stock_price);
        }
        
        println!("Balance: ${}, Stocks: {}", balance, stock_holding);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}