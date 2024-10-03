mod trading_bot;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    trading_bot::run_trading_bot().await;
}
