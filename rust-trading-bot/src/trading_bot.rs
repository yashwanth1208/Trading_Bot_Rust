use chrono::Utc;
use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]

struct StockPrice {
    #[serde(rename = "stockPrice")]
    stock_price: String,
}

#[derive(Debug)]
struct Trade {
    action: String,
    price: f64,
    timestamp: String,
    profit_loss: f64,
}

pub async fn run_trading_bot() {
    let node_api_url = env::var("NODE_API_URL").expect("NODE_API_URL must be set");

    let mut balance: f64 = 10000.0; // Initializing and mutating(Have to be updated) balance in INR
    let mut stock_holding: f64 = 0.0;
    let mut total_profit_loss: f64 = 0.0;
    let mut trade_history: Vec<Trade> = Vec::new();
    let max_iterations = 10;

    for _ in 0..max_iterations {
        let response: StockPrice = reqwest::get(&format!("{}/stock-price", node_api_url))
            .await
            .expect("Failed to fetch stock price")
            .json()
            .await
            .expect("Invalid JSON response");

        let stock_price: f64 = response
            .stock_price
            .parse()
            .expect("Failed to parse stock price");

        println!("Fetched stock price: ₹{}", stock_price);

        if stock_holding == 0.0 && stock_price <= 105.0 {
            stock_holding = balance / stock_price;
            balance = 0.0;
            let timestamp = Utc::now().to_string();
            trade_history.push(Trade {
                action: "BUY".to_string(),
                price: stock_price,
                timestamp,
                profit_loss: 0.0,
            });
            println!("Bought stocks at price: ₹{}", stock_price);
        } else if stock_holding > 0.0 && stock_price >= 106.0 {
            balance = stock_holding * stock_price;
            let profit_loss = balance - 10000.0; 
            total_profit_loss += profit_loss; 
            stock_holding = 0.0;

            let timestamp = Utc::now().to_string();
            trade_history.push(Trade {
                action: "SELL".to_string(),
                price: stock_price,
                timestamp,
                profit_loss,
            });
            println!("Sold stocks at price: ₹{}", stock_price);
            println!("Profit/Loss from this trade: ₹{}", profit_loss);
        }

        println!("Balance: ₹{}, Stocks: {}", balance, stock_holding);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
    print_summary(trade_history, total_profit_loss, balance);
}

// Fun -> summary report
fn print_summary(trade_history: Vec<Trade>, total_profit_loss: f64, final_balance: f64) {
    println!("\n=== Trading Summary ===");
    for trade in trade_history {
        println!(
            "{} at ₹{}, Timestamp: {}, Profit/Loss: ₹{}",
            trade.action, trade.price, trade.timestamp, trade.profit_loss
        );
    }
    println!("Total Profit/Loss: ₹{}", total_profit_loss);
    println!("Final Balance: ₹{}", final_balance);
}
