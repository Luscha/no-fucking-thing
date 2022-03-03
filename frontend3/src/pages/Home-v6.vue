<template>
  <div class="page-wrap">
        <!-- header  -->
        <header class="header-section has-header-main bg-gradient-primary is-theme bg-pattern-2 no-theme-switcher">
          <HeaderMain isTransparent="is-transparent on-dark"></HeaderMain>
          <!-- hero -->
          <HeroSeven></HeroSeven>
        </header>
        <!-- product  -->
        <section class="section-space trending-section bg-primary-dark is-theme">
            <div class="container">
                <!-- section heading -->
                <SectionHeading classname="text-center" :text="SectionData.productData.title"></SectionHeading>
                <!-- product -->
                <ProductsContainerFive></ProductsContainerFive>
                <div class="text-center mt-4 mt-md-5">
                    <ButtonLink :text="SectionData.productData.btnText" :link="SectionData.productData.btnLinkThree" classname="btn-link btn-link-s1"></ButtonLink>
                </div>
            </div><!-- .container -->
        </section><!-- trending-section -->
        <!-- top creators -->
        <section class="top-creator-section section-space bg-gradient-primary is-theme bg-pattern-3">
            <div class="container">
                 <!-- section heading -->
                <SectionHeading classname="text-center" :text="SectionData.creatorData.title"></SectionHeading>
                <!-- creators -->
                <CreatorsThree classname="card-creator-v-wbg card-full"></CreatorsThree>
            </div><!-- .container -->
        </section><!-- end top-creator-section -->
        <!-- top creators -->
        <!-- HowItWork  -->
        <section class="section-space how-it-work-section bg-primary-dark is-theme">
            <div class="container">
                <!-- section heading -->
                <SectionHeading classname="text-center" :text="SectionData.howItWorkData.title"></SectionHeading>
                <HowItWorkItem></HowItWorkItem>
            </div><!-- end container -->
        </section><!-- end how-it-work-section -->
        <!-- Footer  -->
        <Footer classname="bg-primary-darker is-theme"></Footer>
  </div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

import HeroSeven from '@/components/section/HeroSeven.vue'
export default {
  name: 'Home-v6',
  components: {
    HeroSeven
  },
  data () {
    return {
      SectionData
    }
  }
}
</script>