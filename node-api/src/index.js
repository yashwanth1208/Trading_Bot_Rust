require('dotenv').config();
const express = require('express');
const app = express();
const stockService = require('./services/stockService');

app.get('/stock-price', (req, res) => {
    const stockPrice = stockService.generateStockPrice();
    res.json({ stockPrice });
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`Node.js API running on http://localhost:${PORT}`);
});
