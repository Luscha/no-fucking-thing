<template>
  <div class="row g-gs">
    <div :class="{
      'col-xl-3': fullScreen,
      'col-lg-4': fullScreen,
      'col-xl-4': !fullScreen,
      'col-lg-6': !fullScreen,
      }" class="col-sm-6" v-for="nft in nfts" :key="nft.id+nft.contractAddr">
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

    hasMore: {
      type: Boolean,
      default: true
    },

    loading: {
      type: Boolean,
      default: false
    },

    fullScreen: {
      type: Boolean,
      default: true
    }
  },

  mounted() {
    window.onscroll = () => {
      if (this.loading) {
          return;
      }

      let bottomOfWindow = document.documentElement.scrollTop + window.innerHeight > document.documentElement.offsetHeight - 200;
      if (this.hasMore && bottomOfWindow) {
          this.$emit('more')
      }
    }
  }
};
</script>