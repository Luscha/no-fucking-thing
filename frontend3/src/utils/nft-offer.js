// import {safeIpfsToUrl} from "./nft";

export function parseOffering(offering) {
    const denom = {
        uluna: "LUNA",
        uusd: "UST"
    }
    return {
        id: offering.id,
        token_id: offering.token_id,
        price: {
            denom: denom[offering.list_price.denom],
            amount: parseInt(offering.list_price.amount)/1000000,
        },
        contractAddr: offering.contract_addr,
        seller: offering.seller
    }
}