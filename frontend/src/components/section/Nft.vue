<template>
    <div class="card card-full">
        <div class="card-author card-s3 d-flex align-items-center justify-content-between pb-3">
            <div class="d-flex align-items-center">
                <span class="custom-tooltip-wrap card-author-by-wrap">
                    <span class="card-author-by card-author-by-2">Collection</span>
                    <!-- <router-link :to="'/'" class="custom-tooltip author-link">{{ wrapper.collection.name }}</router-link> -->
                    <span class="custom-tooltip author-link">{{ wrapper.collection.name }}</span>
                </span>
            </div>
        </div><!-- end card-author -->
        <div v-if="wrapper.info.image" class="card-image same-h">
            <img :src="wrapper.info.image" class="card-img-top card-img-fit-h" alt="art image">
        </div>
        <div class="card-body p-4">
            <h5 class="card-title text-truncate mb-0">{{ wrapper.info.name }}</h5>
        </div><!-- end card-body -->
        <router-link
            class="details"
            :to="{
                name: 'ProductDetail',
                params: {
                  id: nft.token_id,
                  contract: nft.contractAddr
                }
            }"
        >
        </router-link>
    </div><!-- end card -->
</template>
<script>
import { NftWrapper } from '@/models/nft-wrapper';

export default {
  name: 'Nft',
  props: ['nft'],

  data() {
    return {
      wrapper: new NftWrapper(this.nft.contractAddr, this.nft.token_id)
    }
  },

  mounted() {
    this.wrapper.loadInfo();
    this.wrapper.loadCollection();
  },
}
</script>


<style lang="css" scoped>
 .details {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 1;
 }
</style>