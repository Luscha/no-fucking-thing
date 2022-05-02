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
            <div v-if="product.seller" class="card-author mb-1 d-flex align-items-center">
              <span class="me-1 card-author-by">By</span>
              <router-link :to="'/profile/'+product.seller" class="custom-tooltip author-link link">{{ trunc(product.seller, 18) }}</router-link>
            </div><!-- end card-author -->
            <div class="card-price-wrap d-flex align-items-center justify-content-sm-between mb-3">
                <div class="me-5 me-sm-2">
                    <span class="card-price-title">Price</span>
                    <span class="card-price-number">{{ product.price.amount }} {{ product.price.denom }}</span>
                </div>
                <button v-if="canWithdraw" @click="withdraw" class="btn btn-sm btn-dark">Cancel Listing</button>
                <!-- <div class="text-sm-end">
                    <span class="card-price-title">Current bid</span>
                    <span class="card-price-number">{{ product.priceTwo }} ETH</span>
                </div> -->
            </div><!-- end card-price-wrap -->
            
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
import { exec } from '@/contract/execute';
import { MARKETPLACE_ADDRESS } from '@/config'

import { trunc } from "@/utils/address";
import { NftWrapper } from '@/models/nft-wrapper';
import walletController from "@/mixins/walletController.js"

export default {
  name: 'Products',
  props: ['product'],
  mixins: [walletController],

  data() {
    return {
      wrapper: new NftWrapper(this.product.contractAddr, this.product.token_id)
    }
  },

  computed: {
    canWithdraw() {
      return this.product.seller == this.ConnectedAddress;
    }
  },

  methods: {
    withdraw() {
      const wallet = this.GetWallet();
      if (!wallet) {
          return;
      }          

      exec(wallet, MARKETPLACE_ADDRESS, 
        { withdraw_nft:  { offering_id: this.product.id } },
      )
      .then(() => {
        this.$notify({
            title: 'Success!!',
            text: 'You successfully withdrawn ' + this.wrapper.info.name,
            type: 'success',
        });
        this.$emit("withdraw", this.product.id)
      })
      .catch(err => {
        console.log(err);
        this.$notify({
            title: 'Error occurred',
            text: 'Try to refresh the page and withdraw again',
            type: 'error',
        });
      })
    },

    trunc(str, len) {
      return trunc(str, len)
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
 .author-link,
 .card-price-wrap {
   z-index: 2;
   position: relative;
 }
</style>