//Generating Random Stock Price
module.exports.generateStockPrice = function () {
    return (100 + Math.random() * 10).toFixed(2);
};
