<template>
  <div class="page-wrap">
      <!-- header  -->
      <header class="header-section has-header-main">
        <!-- Header main -->
        <HeaderMain></HeaderMain>
      </header>
      <!-- About section  -->
      <AboutSection class="pt-5 mt-3" :img="SectionData.aboutData.img" isClassThree="col-sm-9 ps-xl-5" isRound="ps-xl-4" isReverse="flex-lg-row-reverse" isClassTwo="pe-lg-5" :title="SectionData.aboutData.title" :content="SectionData.aboutData.content" :contenttwo="SectionData.aboutData.contentTwo" :contentthree="SectionData.aboutData.contentThree"></AboutSection>
  </div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'AboutUs',
  data () {
    return {
      SectionData
    }
  },
}
</script>
