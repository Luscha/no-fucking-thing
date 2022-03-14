import * as query from '@/contract/query'

const LOAD_CHUNK = 20;

export default {
  data() {
    return {
      owned: [],
      ownedIDs: [],
      collectionInfos: {},
      hasMore: true,
      loading: false,
    }
  },

  methods: {
    clear() {
      this.owned = [];
      this.ownedIDs = [];
      this.collectionInfos = {};
      this.hasMore = true;
      this.loading = false;
    },

    loadOwned(owner, start = undefined) {
      if (!owner) {
        return;
      }
      // TODO get all contracts addresses
      query.query("gen0", { contract_info: {} })
      .then(res => {
          this.collectionInfos["gen0"] = res;
          query.query("gen0", { tokens: { owner: owner, limit: LOAD_CHUNK, start_after: start } })
          .then(res => {
              this.ownedIDs = this.ownedIDs.concat(res.tokens);
              this.hasMore = res.tokens == LOAD_CHUNK;
              this.owned = this.owned.concat(res.tokens.map(res => ({contractAddr: "terra1fn9wsg32jpdlz0n84w42fdf593u0tjlkcmjvqa", token_id: res})));
          })
          .finally(() => this.loading = false);
      })
      .catch(err => console.log(err));
    }
  },
}