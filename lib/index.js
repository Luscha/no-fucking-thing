module.exports = ({ wallets, refs, config, client }) => ({
  cw721NumTokens: () => client.query("cw721", { num_tokens: { } }),
  cw721Mint: (id, name, image, description, signer = wallets.owner) => 
  client.execute(signer, "cw721", { mint: { token_id: id, name, image, description, owner: "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v" } })
  ,
  cw721TokenInfo: (id) => client.query("cw721", {nft_info: {token_id : id}}),
  cw721Owner: (id) => client.query("cw721", {owner_of: {token_id : id}}),
  cw721Info: () => client.query("cw721", { contract_info: { } }),
  cw721Minter: () => client.query("cw721", { minter: { } }),
  mpCreateOffer: (signer = wallets.owner) => client.execute(signer, "cw721", { send_nft: {
    contract: "terra1kyl8f2xkd63cga8szgkejdyvxay7mc7qpdc3c5",
    token_id: "test",
    msg: Buffer.from(JSON.stringify({
      list_price: {
        denom: "uluna",
        amount: "50",
      }
    })).toString("base64")
  }}),
  mpWithdrawNft: (offering_id, signer = wallets.owner) => client.execute(signer, "marketplace", { withdraw_nft: {offering_id} }),
  mpGetOfferings: () => client.query("marketplace", { get_offerings: {} }),
  mpBuy: (offering_id, signer = wallets.customer) => {
    return client.execute(signer, "marketplace", { buy: { msg: 
      {
        offering_id, payment: {denom: "uluna", amount: "50"} 
      }
    }},
    { uluna: 50})
  },
  getCount: () => client.query("counter", { get_count: {} }),
  increment: (signer = wallets.owner) =>
    client.execute(signer, "counter", { increment: {} }),
});
