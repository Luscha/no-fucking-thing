<template>
    <div>
      <div class="row g-4">
        <p class="col-md-6 col-sm-12 smaller mb-2">Not finding your NFT's? Try to add the contract address to our database in order to find information about them.</p>
        <div class="col-md-6 col-sm-12">
          <div :style="{display: 'flex'}">
            <input type="text" class="form-control form-control-s1" placeholder="NFT contract address">
            <button class="btn btn-dark">Import</button>
          </div>
        </div>
      </div>
      <hr class="my-4">
      <NftsContainer
        :nfts="owned" 
        :loading="loading" 
        :hasMore="hasMore"
        :fullScreen="false"
        @more="more()"/>
    </div><!-- row -->
</template>

<script>
import token from '@/mixins/token-owned';

export default {
  name: 'ProfileOwned',
  mixins: [token],

  props: {
    address: {
      type: String,
      required: true,
    },
  },

  data () {
    return {
    }
  },

  watch: {
    'address': {
      handler(address) {
        this.clear(),
        this.loadOwned(address);
      }
    }
  },

  methods: {
    more() {
      this.loading = true;
      this.loadOwned(this.address, this.ownedIDs[this.ownedIDs.length-1]);
    }
  },

  mounted() {
    this.loading = true;
    this.loadOwned(this.address);
  }
}
</script>