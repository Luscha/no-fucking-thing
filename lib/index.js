module.exports = ({ wallets, refs, config, client }) => ({
  gen0NumTokens: () => client.query("gen0", { num_tokens: { } }),
  gen0Mint: (id, name, description, image, owner, signer = wallets.owner) => 
  client.execute(signer, "gen0", { mint: { token_id: id, name, description, image, owner } }),
  gen0TokenInfo: (id) => client.query("gen0", {nft_info: {token_id : id}}),
  gen0Owner: (id) => client.query("gen0", {owner_of: {token_id : id}}),
  gen0Info: () => client.query("gen0", { contract_info: { } }),
  gen0Minter: () => client.query("gen0", { minter: { } }),
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
    return client.execute(signer, "gen0", { send_nft: payload});
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