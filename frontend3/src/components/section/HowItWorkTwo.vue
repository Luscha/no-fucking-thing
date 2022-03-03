<template>
  <section class="section-space how-it-work-section">
      <div class="container">
          <!-- section heading -->
          <SectionHeading classname="text-center" :text="SectionData.howItWorkData.titleThree"></SectionHeading>
          <!-- tab -->
          <Tab></Tab>
      </div><!-- end container -->
  </section><!-- end how-it-work-section -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'HowItWorkTwo',
  data () {
    return {
      SectionData
    }
  }
}
</script>