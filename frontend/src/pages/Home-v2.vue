<template>
  <div class="page-wrap">
        <!-- header  -->
        <header class="header-section has-header-main bg-gradient-2">
          <!-- Header main -->
          <HeaderMain isTransparent="is-transparent"></HeaderMain>
          <!-- hero -->
          <div class="hero-wrap hero-wrap-2 section-space">
            <div class="container">
                <div class="row flex-lg justify-content-between align-items-center">
                    <div class="col-lg-4">
                        <div class="hero-content pt-lg-0 pb-0 mt-lg-n4">
                            <h1 class="hero-title hero-title-s1 mb-3">{{ strings.leftTitle }}</h1>
                            <p class="hero-text mb-4 pb-1">{{ strings.leftContent }}</p>
                            <!-- button group -->
                            <ButtonGroup :btns="btnDataLeft" classname="hero-btns"></ButtonGroup>
                        </div><!-- hero-content -->
                    </div><!-- col-lg-6 -->
                    <div class="col-lg-4">
                        <img :src="`https://multiplayer.net-cdn.it/thumbs/images/2022/01/06/jinx-arcane-netflix-cosplay_jpg_1400x0_q85.jpg`" class="mb-3 card-img-top" alt="art image">
                        <!-- Featured Item Slider -->
                        <FeaturedItemSlider :featured="featuredExtended"></FeaturedItemSlider>
                        <ButtonGroup :btns="btnDataCenter" classname="hero-btns"></ButtonGroup>
                    </div><!-- end col-lg-5 -->
                    <div class="col-lg-4">
                        <div class="hero-content pt-lg-0 pb-0 mt-lg-n4">
                            <h1 class="hero-title hero-title-s1 mb-3">{{ strings.rightTitle }}</h1>
                            <p class="hero-text mb-4 pb-1">{{ strings.rightContent }}</p>
                            <!-- button group -->
                            <ButtonGroup :btns="btnDataRight" classname="hero-btns"></ButtonGroup>
                        </div><!-- hero-content -->
                    </div><!-- col-lg-6 -->
                </div><!-- end row -->
            </div><!-- .container-->
        </div><!-- end hero-wrap -->
        </header>
        <!-- HowItWork  -->
        <HowItWork classname="col-lg-3" gutterBottom="mb-3"></HowItWork>
        <!-- funFact  -->
        <!-- <funFactSection :isBg="true" class="section-space" classname="col-lg-4 col-sm-6" :items="SectionData.funfactData.funfactList"></funFactSection> -->
        <!-- Footer  -->
        <Footer classname="bg-dark on-dark"></Footer>
  </div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import {parseOffering} from '@/utils/nft-offer'
import * as query from '@/contract/query'
import { MARKETPLACE_ADDRESS } from '@/config'

export default {
  name: 'Home-v2',
  data () {
    return {
      SectionData,
      offerings: [],
      btnDataLeft: [    
        {
          btnClass: 'btn-lg btn-grad',
          title: 'Mint Gen0',
          path: 'mint-gen0'
        },
      ],

      btnDataCenter: [
        {
          btnClass: 'btn-lg btn-outline-dark',
          title: 'Litepaper',
          path: '/litepaper'
        }
      ],

      btnDataRight: [
        {
          btnClass: 'btn-lg btn-outline-dark',
          title: 'Learn More',
          path: '/learn-more'
        },
      ],

      strings: {
        leftTitle: "Support the project:",
        leftContent: "Buying the Gen0 you will be an active member of its growth and you will be rewarded with part of the future earnings",
        rightTitle: "Our idea:",
        rightContent: "If you are intereste in our project and its future development",
      }
    }
  },
  
  mounted() {
    query.query(MARKETPLACE_ADDRESS, { offerings: { limit: 4 } }).then(res => {
      this.offerings = res.offerings.map(nft => (parseOffering(nft)));
    });
  }
}
</script>