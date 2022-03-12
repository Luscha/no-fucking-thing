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
        @more="more()"/>
    </div><!-- row -->
</template>

<script>
export default {
  name: 'ProfileOwned',
  props: {
    address: {
      type: String,
      required: true,
    },
  },

  data () {
    return {
      loading: false,
      hasMore: true,
      owned: []
    }
  },

  methods: {
    more() {
      this.loading = true;
    }
  }
}
</script>