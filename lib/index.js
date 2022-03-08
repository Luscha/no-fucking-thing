module.exports = ({ wallets, refs, config, client }) => ({
  cw721NumTokens: () => client.query("cw721", { num_tokens: { } }),
  cw721Mint: (id, name, description, image, owner, signer = wallets.owner) => 
  client.execute(signer, "cw721", { mint: { token_id: id, name, description, image, owner } }),
  cw721TokenInfo: (id) => client.query("cw721", {nft_info: {token_id : id}}),
  cw721Owner: (id) => client.query("cw721", {owner_of: {token_id : id}}),
  cw721Info: () => client.query("cw721", { contract_info: { } }),
  cw721Minter: () => client.query("cw721", { minter: { } }),
  mpCreateOffer: (contract, token_id, priceDenom, priceAmount, signer = wallets.owner) => {
    const payload = {
      contract,
      token_id,
      msg: Buffer.from(JSON.stringify({
        list_price: {
          denom: priceDenom,
          amount: priceAmount.toString(),
        }
      })).toString("base64")
    };
    return client.execute(signer, "cw721", { send_nft: payload});
  },
  mpWithdrawNft: (offering_id, signer = wallets.owner) => client.execute(signer, "marketplace", { withdraw_nft: {offering_id} }),
  mpGetOfferings: () => client.query("marketplace", { get_offerings: {} }),
  mpBuy: (offering_id, signer = wallets.customer) => {
    return client.execute(signer, "marketplace", { buy: { msg: 
      {
        offering_id, payment: {denom: "uluna", amount: "50"} 
      }
    }},
    {uluna: 50})
  },
});

// await lib.mpCreateOffer("terra1l425neayde0fzfcv3apkyk4zqagvflm6cmha9v", "GEN0-1", "uluna", 1000)