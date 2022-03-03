<template>
  <div class="page-wrap">
        <!-- header  -->
        <header class="header-section has-header-main">
          <HeaderMain></HeaderMain>
          <!-- hero -->
          <HeroEight></HeroEight>
        </header>
        <!-- product  -->
        <section class="section-space trending-section bg-gray">
            <div class="container">
                <!-- section heading -->
                <SectionHeading classname="text-center" :text="SectionData.productData.title"></SectionHeading>
                <!-- product -->
                <ProductsContainerSix></ProductsContainerSix>
                <div class="text-center mt-4 mt-md-5">
                    <ButtonLink :text="SectionData.productData.btnText" :link="SectionData.productData.btnLinkFive" classname="btn-link btn-link-s1"></ButtonLink>
                </div>
            </div><!-- .container -->
        </section><!-- trending-section -->
        <!-- top creators -->
        <section class="top-creator-section section-space">
            <div class="container">
                 <!-- section heading -->
                <SectionHeading classname="text-center mb-lg-3" :text="SectionData.creatorData.title"></SectionHeading>
                <!-- creators -->
                <CreatorsFour></CreatorsFour>
            </div><!-- .container -->
        </section><!-- end top-creator-section -->
        <!-- How it work  -->
        <HowItWorkFour :title="SectionData.howItWorkData.title"></HowItWorkFour>
        <!-- Footer  -->
        <Footer classname="bg-dark on-dark"></Footer>
  </div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import HeroEight from '@/components/section/HeroEight.vue'
export default {
  name: 'Home-v7',
  components: {
    HeroEight
  },
  data () {
    return {
      SectionData
    }
  }
}
</script>