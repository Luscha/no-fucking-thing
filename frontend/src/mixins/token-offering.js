import * as query from '@/contract/query'
import { parseOffering } from '@/utils/nft-offer'
import { MARKETPLACE_ADDRESS } from '@/config'

const LOAD_CHUNK = 20;

export default {
  data() {
    return {
      offerings: [],
      hasMore: true,
      loading: false
    }
  },

  methods: {
    clear() {
      this.offerings = [];
    },

    loadOfferings(owner, start = undefined) {
      if (!owner) {
        return;
      }
      query.query(MARKETPLACE_ADDRESS, { offerings_by_owner: { owner: owner, limit: LOAD_CHUNK, start_after: start } })
        .then(res => {
          this.offerings = this.offerings.concat(res.offerings.map(nft => (parseOffering(nft))));
          this.hasMore = this.offerings.length == LOAD_CHUNK;
        })
        .finally(() => this.loading = false);
    }
  },
}