const { task } = require("@iboss/terrain");
const lib = require("../lib");
const config = require("../refs.terrain.json");

const contractAdress = (chain, contract) => config[chain][contract].contractAddresses.default;

function getRandomInt(max) {
  return Math.floor(Math.random() * max);
}

task(async (env) => {
  const { mpCreateOffer } = lib(env);
  const contract = contractAdress("localterra", "marketplace");
  for(let i = 0; i < 50; i++) {
    const denom = getRandomInt(2) == 0 ? "uusd" : "uluna";
    const price = 3000000 + getRandomInt(10000000); // 3 + 10 amount
    const name = "GEN0-"+i;
    console.log(contract, name, denom, price)
    const res = await mpCreateOffer(contract, name, denom, price).catch(err => console.log(err))
    console.log(res)
  }
});
