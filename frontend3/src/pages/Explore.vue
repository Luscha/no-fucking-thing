<template>
<div class="page-wrap">
    <!-- header  -->
    <header class="header-section has-header-main">
        <!-- Header main -->
        <HeaderMain></HeaderMain>
    </header>
    <!-- Explore section  -->
    <ExploreSection></ExploreSection>
    <!-- top creators -->
    <TopCreators classname="section-space-t"></TopCreators>
    <!-- Collections -->
    <Collections></Collections>
    <!-- Footer  -->
    <Footer classname="bg-dark on-dark"></Footer>
</div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'Explore',
  
  data () {
    return {
      SectionData
    }
  }
}
</script>
