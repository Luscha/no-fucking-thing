import * as query from '@/contract/query'
import axios from 'axios';
import { BACKEND_REST_API_ENDPOINT } from "@/config"

const LOAD_CHUNK = 10;

export default {
  data() {
    return {
      owned: [],
      collectionInfos: [],
      cursor: {},
      hasMore: true,
      loading: false,
    }
  },

  methods: {
    clear() {
      this.owned = [];
      this.hasMore = true;
      this.loading = false;
      this.cursor = {}
    },

    async loadContracts() {
      await axios.get(BACKEND_REST_API_ENDPOINT + "/collections")
      .then(response => {
        response.data.forEach(c => {
          this.collectionInfos.push({address: c.contractAddress, minter: c.minter, name: c.name})
        });
        console.log(this.collectionInfos)
      }).catch(err => console.log(err))
    },

    async loadOwned(owner) {
      if (!owner) {
        return;
      }
      let loaded = 0;
      let searchIn = this.collectionInfos;
      if (this.cursor.contract) {
        searchIn = searchIn.slice(this.collectionInfos.indexOf(c => c.address == this.cursor.contract), searchIn.length);
      }
      
      for (let collection of searchIn) {
        const start = this.cursor.contract == collection.address ? this.cursor.token_id : undefined;
        let tokens = await query.query(collection.address, { tokens: { owner: owner, limit: LOAD_CHUNK, start_after: start } });
        tokens = tokens.tokens.slice(0, LOAD_CHUNK-loaded);
        loaded += tokens.length;

        this.owned = this.owned.concat(tokens.map(res => ({contractAddr: collection.address, token_id: res})));
        this.cursor = { contract:collection.address, token_id: tokens[tokens.length-1] };

        if (loaded >= LOAD_CHUNK) {
          this.loading = false;
          this.hasMore = true;
          return;
        }
      }

      this.loading = false;
      this.hasMore = false;
    }
  },
}