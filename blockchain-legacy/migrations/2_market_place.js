const MarketPlace = artifacts.require("MarketPlace");

module.exports = function(_deployer) {
  _deployer.deploy(MarketPlace)
};
