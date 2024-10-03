//Generating Random Stock Prize
module.exports.generateStockPrice = function () {
    return (100 + Math.random() * 10).toFixed(2);
};
