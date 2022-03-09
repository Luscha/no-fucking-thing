<template>
    <div class="card card-full">
        <div v-if="wrapper.info.image" class="card-image same-h">
            <img :src="wrapper.info.image" class="card-img-top card-img-fit-h" alt="art image">
        </div>
        <div class="card-body p-4">
            <h5 class="card-title text-truncate mb-0">{{ wrapper.info.name }}</h5>
            <div class="card-author mb-1 d-flex align-items-center">
                <span class="me-1 card-author-by">By</span>
                <router-link :to="'/author/'+product.seller" class="custom-tooltip author-link">{{ trunc(product.seller, 18) }}</router-link>
            </div><!-- end card-author -->
            <div class="card-price-wrap d-flex align-items-center justify-content-sm-between mb-3">
                <div class="me-5 me-sm-2">
                    <span class="card-price-title">Price</span>
                    <span class="card-price-number">{{ product.price.amount }} {{ product.price.denom }}</span>
                </div>
                <!-- <div class="text-sm-end">
                    <span class="card-price-title">Current bid</span>
                    <span class="card-price-number">{{ product.priceTwo }} ETH</span>
                </div> -->
            </div><!-- end card-price-wrap -->
            <span class="btn btn-sm btn-dark">Buy</span>
        </div><!-- end card-body -->
        <router-link
            class="details"
            :to="{
                name: 'ProductDetail',
                params: {
                  id: product.token_id,
                  contract: product.contractAddr
                }
            }"
        >
        </router-link>
    </div><!-- end card -->
</template>
<script>
import { trunc } from "@/utils/address";
import { NftWrapper } from '@/models/nft-wrapper';

export default {
  name: 'Products',
  props: ['product'],
  methods: {
    trunc(str, len) {
      return trunc(str, len)
    }
  },

  data() {
    return {
      wrapper: new NftWrapper(this.product.contractAddr, this.product.token_id)
    }
  },

  computed: {
    loaded() {
      return this.extension.image != undefined;
    },
  },

  mounted() {
    this.wrapper.loadInfo();
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
 .author-link,
 .card-price-wrap {
   z-index: 2;
   position: relative;
 }
</style>