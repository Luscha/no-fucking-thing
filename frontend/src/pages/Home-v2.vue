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
                        <!-- Featured Item Slider -->
                        <FeaturedItemSlider :featured="featured"></FeaturedItemSlider>
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
        <!-- Footer  -->
        <Footer classname="bg-dark on-dark"></Footer>
  </div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import { NftWrapper } from '@/models/nft-wrapper';
import * as query from '@/contract/query'
import { GEN0_ADDRESS } from '@/config'

export default {
  name: 'Home-v2',
  data () {
    return {
      SectionData,
      featured: [],
      btnDataLeft: [    
        {
          btnClass: 'btn-lg btn-grad',
          title: 'Mint Gen0',
          path: 'mint-gen0'
        },
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
        {
          btnClass: 'btn-lg btn-outline-dark',
          title: 'FAQ',
          path: '/faq'
        },
      ],

      strings: {
        leftTitle: "Support the project",
        leftContent: "Buying the Gen0 you will be an active member of its growth and you will be rewarded with part of the future earnings",
        rightTitle: "Our idea",
        rightContent: "If you are interested in our project and its future development",
      }
    }
  },
  
  mounted() {
    query.query(GEN0_ADDRESS, { all_tokens: { start_after: "6", limit: 4 } }).then(res => {
      
      this.featured = res.tokens?.map(nft => (new NftWrapper(GEN0_ADDRESS, nft)));
      for (let i = 0; i < this.featured.length; i++) {
        this.featured[i].loadCollection();
        this.featured[i].loadInfo(false);
      }
      console.log(this.featured)
    });
  }
}
</script>