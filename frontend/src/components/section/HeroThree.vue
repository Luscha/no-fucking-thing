<template>
    <div class="hero-wrap hero-wrap-2 section-space">
        <div class="container">
            <div class="row flex-lg-row-reverse justify-content-between align-items-center">
                <div class="col-lg-5">
                    <!-- Featured Item Slider -->
                    <FeaturedItemSlider :featured="featuredExtended"></FeaturedItemSlider>
                </div><!-- end col-lg-5 -->
                <div class="col-lg-6">
                    <div class="hero-content pt-lg-0 pb-0 mt-lg-n4">
                        <h1 class="hero-title hero-title-s1 mb-3">{{ SectionData.heroDataThree.title }}</h1>
                        <p class="hero-text mb-4 pb-1">{{ SectionData.heroDataThree.content }}</p>
                        <!-- button group -->
                        <ButtonGroup :btns="SectionData.btnDataFour" classname="hero-btns"></ButtonGroup>
                    </div><!-- hero-content -->
                </div><!-- col-lg-6 -->
            </div><!-- end row -->
        </div><!-- .container-->
    </div><!-- end hero-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

import { safeIpfsToUrl } from "@/utils/nft";
import * as query from '@/contract/query'

export default {
  name: 'HeroThree',
  props: {
    featuredProducts: {
      type: Array,
      default() {
        return SectionData.productData.products
      },
    },
  },
  data () {
    return {
      SectionData,
      extension: {}
    }
  },

  computed: {
    featuredExtended () {
      return this.featuredProducts.map(p => ({...p, ...this.extension[p.token_id]}))
    }
  },

  beforeUpdate() {
    for (let prod of this.featuredProducts) {
      if (!this.extension[prod.token_id]?.name && prod.contractAddr) {
        query.queryRaw(prod.contractAddr, { nft_info: {token_id: prod.token_id} })
        .then(res => {
          this.extension[prod.token_id] = {...res, image: safeIpfsToUrl(res.image)}
        })
      }
    }
  },
}
</script>
