<template>
  <div class="page-wrap">
        <!-- header  -->
        <header class="header-section has-header-main bg-gradient-2">
          <!-- Header main -->
          <HeaderMain isTransparent="is-transparent"></HeaderMain>
          <!-- hero -->
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
                            <ButtonGroup :btns="btnDataFour" classname="hero-btns"></ButtonGroup>
                        </div><!-- hero-content -->
                    </div><!-- col-lg-6 -->
                </div><!-- end row -->
            </div><!-- .container-->
        </div><!-- end hero-wrap -->
        </header>
        <section class="section-space trending-section bg-gray">
            <div class="container">
                <!-- section heading -->
                <SectionHeading classname="text-center" :text="SectionData.productData.title" :content="SectionData.productData.content" isMargin="mb-3"></SectionHeading>
                <!-- product -->
                <ProductsContainer :products="offerings"></ProductsContainer>
                <div class="text-center mt-4 mt-md-5">
                    <ButtonLink :text="SectionData.productData.btnText" :link="SectionData.productData.btnLink" classname="btn-link btn-link-s1"></ButtonLink>
                </div>
            </div><!-- .container -->
        </section><!-- trending-section -->
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
      btnDataFour: [    
        {
          btnClass: 'btn-lg btn-grad',
          title: 'Mint Gen0',
          path: 'mint-gen0'
        },
        {
          btnClass: 'btn-lg btn-outline-dark',
          title: 'Learn More',
          path: '/learn-more'
        },
        {
          btnClass: 'btn-lg btn-outline-dark',
          title: 'Litepaper',
          path: '/litepaper'
        }
      ],
    }
  },
  
  mounted() {
    query.query(MARKETPLACE_ADDRESS, { offerings: { limit: 4 } }).then(res => {
      this.offerings = res.offerings.map(nft => (parseOffering(nft)));
    });
  }
}
</script>