<template>
<div class="page-wrap">
    <!-- header  -->
    <header class="header-section has-header-main bg-gradient-primary is-theme bg-pattern-2">
        <!-- Header main -->
        <HeaderMain isTransparent="is-transparent on-dark"></HeaderMain>
        <!-- hero -->
        <HeroFour classnameTwo="sub-header-2" classname="hero-title" :title="SectionData.breadcrumbData.breadcrumbList.title" :lists="SectionData.breadcrumbData.breadcrumbList.navList"></HeroFour>
    </header>
    <!-- Explore section  -->
    <ExploreSectionFour></ExploreSectionFour>
    <section class="top-creator-section section-space bg-gradient-primary is-theme bg-pattern-3">
        <div class="container">
              <!-- section heading -->
            <SectionHeading :text="SectionData.creatorData.title"></SectionHeading>
            <!-- creators -->
            <CreatorsThree classname="card-creator-v-wbg card-full"></CreatorsThree>
        </div><!-- .container -->
    </section><!-- end top-creator-section -->
    <!-- Collections -->
    <CollectionsTwo></CollectionsTwo>
    <!-- Footer  -->
    <Footer classname="bg-primary-darker is-theme"></Footer>
</div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import HeroFour from '@/components/section/HeroFour.vue'
export default {
  name: 'Explore-v4',
  components: {
    HeroFour
  },
  data () {
    return {
      SectionData
    }
  }
}
</script>
