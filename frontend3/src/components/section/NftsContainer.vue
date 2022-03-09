<template>
  <div class="row g-gs">
    <div class="col-xl-3 col-lg-4 col-sm-6" v-for="nft in nfts" :key="nft.id+nft.contractAddr">
        <Nft :nft="nft"></Nft>
    </div>
  </div>
</template>

<script>
import Nft from '@/components/section/Nft'
 
export default {
  name: 'NftsContainer',
  components: { 
    Nft 
  },
  props: {
    nfts: {
      type: Array,
      default() {
        return []
      },
    },
  },
};
</script>