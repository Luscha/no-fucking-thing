const { task } = require("@iboss/terrain");
const lib = require("../lib");

task(async (env) => {
  const { gen0Mint } = lib(env);
  for(let i = 0; i < 50; i++) {
    const name = "GEN0-"+i;
    const image = "ipfs://bafybeihjpofahnzibqg25nuguwh7ssusohh5nzexocymwglehbzqv3lvzi"
    const res = await gen0Mint(name, name, "Nft number "+name+" of the GENERATION 0 of no-fucking-thing", image, "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v")
    console.log(res)
  }
});
