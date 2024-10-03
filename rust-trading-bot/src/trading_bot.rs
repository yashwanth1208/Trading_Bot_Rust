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
    amount: f64,
}

pub async fn run_trading_bot() {
    let node_api_url = env::var("NODE_API_URL").expect("NODE_API_URL must be set");

    let mut balance: f64 = 10000.0; // Initial balance in INR
    let mut stock_holding: f64 = 0.0;
    let mut total_profit_loss: f64 = 0.0;
    let mut trade_history: Vec<Trade> = Vec::new();
    let mut last_purchase_price: f64 = 0.0;

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

        //Buying the stock when price drops by 2%
        if stock_holding == 0.0
            && (last_purchase_price == 0.0 || stock_price <= last_purchase_price * 0.98)
        {
            stock_holding = balance / stock_price;
            last_purchase_price = stock_price;
            balance = 0.0;
            let timestamp = Utc::now().to_string();
            trade_history.push(Trade {
                action: "BUY".to_string(),
                price: stock_price,
                timestamp,
                amount: 0.0,
            });
            println!("Bought stocks at price: ₹{}", stock_price);
        }

        //selling the stock when price raise by 3%
        if stock_holding > 0.0 && stock_price >= last_purchase_price * 1.03 {
            let sale_amount = stock_holding * stock_price;
            let profit_loss = sale_amount - (last_purchase_price * stock_holding);
            total_profit_loss += profit_loss;
            balance = sale_amount;
            stock_holding = 0.0;

            let timestamp = Utc::now().to_string();
            trade_history.push(Trade {
                action: "SELL".to_string(),
                price: stock_price,
                timestamp,
                amount: profit_loss,
            });
            println!("Sold stocks at price: ₹{}", stock_price);
            println!(
                "{} ₹{:.2}",
                if profit_loss >= 0.0 {
                    "Profit:"
                } else {
                    "Loss:"
                },
                profit_loss.abs()
            );
        }

        println!("Balance: ₹{}, Stocks: {}", balance, stock_holding);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    print_summary(trade_history, total_profit_loss, balance);
}

// Func -> summary report
fn print_summary(trade_history: Vec<Trade>, total_profit_loss: f64, final_balance: f64) {
    println!("\n=== Trading Summary ===");
    for trade in trade_history {
        let formatted_amount = if trade.amount >= 0.0 {
            format!("+ ₹{:.2}", trade.amount)
        } else {
            format!("- ₹{:.2}", trade.amount.abs())
        };

        println!(
            "{} at ₹{}, Timestamp: {}, Amount: {}",
            trade.action, trade.price, trade.timestamp, formatted_amount
        );
    }

    let total_formatted = if total_profit_loss >= 0.0 {
        format!("Total Profit: + ₹{:.2}", total_profit_loss)
    } else {
        format!("Total Loss: - ₹{:.2}", total_profit_loss.abs())
    };

    println!("{}", total_formatted);
    println!("Final Balance: ₹{}", final_balance);
}
