
import { safeIpfsToUrl } from "@/utils/nft";
import { parseOffering } from "@/utils/nft-offer";
import * as query from '@/contract/query'

export class NftWrapper {
    contract = "";
    token_id = "";
    loaded = false;
    inSale = false;

    info = {};
    owner = {};
    collection = {};
    minter = {};
    offer = {};

    constructor(contract, token_id) {
        this.contract = contract;
        this.token_id = token_id;
    }

    load = function(force = false) {
        this.loadInfo(force);
        this.loadCollection();
        this.loadOffer();
    }

    loadInfo = function(force) {
        this.loaded = true;
        query.queryRaw(this.contract, { all_nft_info: {token_id: this.token_id} })
        .then(res => {
            this.info = {...res.info, image: safeIpfsToUrl(res.info.image)}

            // Do not override offer seller
            if (!force && !this.owner.address) {
                this.owner = {address: res.access.owner}
            }
        })
        .catch(err => console.log(err));
    }

    loadCollection = function() {
        this.loaded = true;
        query.queryRaw(this.contract, { contract_info: {} })
        .then(res => {
            this.collection = res
        })
        .catch(err => console.log(err));

        query.queryRaw(this.contract, { minter: {} })
        .then(res => {
            this.minter = {address: res.minter}
        })
        .catch(err => console.log(err));
    }

    loadOffer = function() {
        query.query("marketplace", { offering_by_nft: {
            collection_contract: this.contract,
            token_id: this.token_id,
        } })
        .then(res => {
            this.offer = parseOffering(res.offering);
            this.inSale = true;
            this.owner = {address: this.offer.seller};
        })
        .catch(err => console.log(err));
    }
}