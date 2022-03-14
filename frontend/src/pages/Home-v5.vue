<template>
  <div class="page-wrap">
        <!-- header  -->
        <header class="header-section has-header-main bg-gradient">
          <HeaderMain></HeaderMain>
          <!-- hero -->
          <HeroSix></HeroSix>
        </header>
        <!-- HowItWork  -->
        <HowItWorkThree></HowItWorkThree>
        <!-- product  -->
        <section class="section-space trending-section bg-gray">
            <div class="container">
                <!-- section heading -->
                <SectionHeadingTwo :meta="SectionData.productData.meta" :text="SectionData.productData.title" :content="SectionData.productData.contentTwo"></SectionHeadingTwo>
                <!-- product -->
                <ProductsContainerThree></ProductsContainerThree>
                <div class="text-center mt-4 mt-md-5">
                    <ButtonLink :text="SectionData.productData.btnText" :link="SectionData.productData.btnLinkTwo" classname="btn-link btn-link-s1"></ButtonLink>
                </div>
            </div><!-- .container -->
        </section><!-- trending-section -->
        <!-- top creators -->
        <section class="top-creator-section section-space">
            <div class="container">
                 <!-- section heading -->
                <SectionHeadingTwo :meta="SectionData.creatorData.meta" :text="SectionData.creatorData.title" :content="SectionData.creatorData.content"></SectionHeadingTwo>
                <!-- creators -->
                <CreatorsThree></CreatorsThree>
            </div><!-- .container -->
        </section><!-- end top-creator-section -->
        <!-- top creators -->
        <section class="sell-buy-section section-space bg-gray">
            <div class="container">
                <SellBuy></SellBuy>
            </div><!-- .container -->
        </section><!-- end sell-buy-section -->
        <!-- Footer  -->
        <Footer classname="bg-dark on-dark"></Footer>
  </div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

import HeroSix from '@/components/section/HeroSix.vue'
export default {
  name: 'Home-v5',
  components: {
    HeroSix
  },
  data () {
    return {
      SectionData
    }
  }
}
</script>