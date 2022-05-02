/* eslint-disable no-undef */
<template>
    <swiper 
    :slides-per-view="1"
    :loop="true"
    :navigation="{ clickable: true }" class="swiper-button-s1">
        <swiper-slide v-for="wrapper in featured" :key="wrapper.token_id">
            <div class="card">
                <div v-if="wrapper.info?.image" class="card-image">
                    <img :src="wrapper.info?.image" class="card-img-top" alt="art image">
                </div>
                <div class="card-body d-flex align-items-center">
                    <!-- <div class="avatar flex-shrink-0 me-2">
                        <img :src="product.avatar" class="card-img-top" alt="art image">
                    </div> -->
                    <div>
                        <h5 class="card-title text-truncate mb-0">{{ wrapper.info?.name }}</h5>
                        <div class="card-author d-flex align-items-center">
                            <span class="me-1 card-author-by">By</span>
                            <router-link :to="'/profile/'+wrapper.owner.address" class=" author-link">{{ trunc(wrapper.owner.address, 20) }}</router-link>
                        </div><!-- end card-author -->
                    </div>
                </div><!-- end card-body -->
                <router-link
                  class="details"
                  :to="{
                      name: 'ProductDetail',
                      params: {
                        id: wrapper.token_id,
                        contract: wrapper.contract
                      }
                  }"
              >
              </router-link>
            </div><!-- end card -->
        </swiper-slide>
    </swiper>
</template>
<script>

import { trunc } from "@/utils/address";

// core version + navigation, pagination modules:
import SwiperCore, { Navigation } from 'swiper';

// configure Swiper to use modules
SwiperCore.use([Navigation]);

// Import Swiper Vue.js components
import { Swiper, SwiperSlide } from 'swiper/vue';

export default {
  name: 'FeaturedItemSlider',
  components: {
      Swiper,
      SwiperSlide,
  },

  props : {
    featured: {
      type: Array,
      default() {
        return []
      },
    },
  },

  methods: {
    trunc(str, len) {
      return trunc(str, len)
    }
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
 .author-link {
   z-index: 2;
   position: relative;
 }
</style>