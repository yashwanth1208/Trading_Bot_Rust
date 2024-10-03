# Trading_Bot_Rust

## Overview
This repo is a simple trading bot implemented in **Node.js** and **Rust**. The bot fetches stock prices from a Node.js API and makes trading decisions based on predefined conditions. It tracks profit and loss, as well as provides a summary report at the end of the trading session.

# Project overview and setup instructions
## Mock API

### Overview

The Node.js Mock API serves stock price data for the trading bot. It exposes a single endpoint to provide the current stock price.

### Setup

1. **Install Dependencies**: Run the following command to install the required packages:

   ```bash
   npm install express cors
2. **Start the Server**: Use the command below to start the API server:

   ```bash
   npm start
3. **API Endpoint**: The server runs on http://localhost:3000/stock-price. The stock price is returned in JSON format.

## Rust Backend
### Overview

The Rust trading bot interacts with the Node.js API to fetch stock prices and make buy/sell decisions based on price fluctuations.

### Setup

1. **Install Rust**: Install rust on your local machine from the rust official website.

2. **Build The Project**: Navigate to the Rust project directory and build the project:

   ```bash
    cargo build
3. **Run the Bot**: Excecute the following command to start the exceution of trading bot:
   ```bash
    cargo run
## Trading Logic
1.   **Buy Condition**: The bot buys stocks when the price drops by 2% from the last purchase price.
2.   **Sell Condition**: The bot sells stocks when the price rises by 3% from the last purchase price.
3.   **Profit/Loss Tracking**: The bot tracks the profit or loss from each trade and displays a summary report at the end of the trading session.
   
## Sample Output
   ```bash
   Fetched stock price: ₹105.26
  Balance: ₹10000, Stocks: 0
  Fetched stock price: ₹108.52
Balance: ₹10000, Stocks: 0
Fetched stock price: ₹104.99
Bought stocks at price: ₹104.99
Balance: ₹0, Stocks: 95.2471663967997
Fetched stock price: ₹108.98
Sold stocks at price: ₹108.98
Profit: ₹380.04
Balance: ₹10380.036193923232, Stocks: 0
Fetched stock price: ₹100.26
Bought stocks at price: ₹100.26
Balance: ₹0, Stocks: 103.53118086897298
Fetched stock price: ₹109.08
Sold stocks at price: ₹109.08
Profit: ₹1293.18
Balance: ₹11293.181209187573, Stocks: 0
Fetched stock price: ₹108.55
Balance: ₹11293.181209187573, Stocks: 0
Fetched stock price: ₹106.22
Balance: ₹11293.181209187573, Stocks: 0
Fetched stock price: ₹107.65
Balance: ₹11293.181209187573, Stocks: 0
Fetched stock price: ₹102.04
Bought stocks at price: ₹102.04
Balance: ₹0, Stocks: 110.67406124252814

=== Trading Summary ===
BUY at ₹104.99, Timestamp: 2024-10-03 20:04:45.417744 UTC, Amount: + ₹0.00
SELL at ₹108.98, Timestamp: 2024-10-03 20:04:55.425840 UTC, Amount: + ₹380.04
BUY at ₹100.26, Timestamp: 2024-10-03 20:05:05.433115 UTC, Amount: + ₹0.00
SELL at ₹109.08, Timestamp: 2024-10-03 20:05:15.441292 UTC, Amount: + ₹1293.18
BUY at ₹102.04, Timestamp: 2024-10-03 20:05:55.473831 UTC, Amount: + ₹0.00
Total Profit: + ₹1673.22
Final Balance: ₹0
