<template>
  <section class="section-space-b how-it-work-section">
      <div class="container">
          <!-- section heading -->
          <SectionHeading classname="text-center" :text="title" :content="subtitle" :isMargin="gutterBottom"></SectionHeading>
          <!-- How It Work Slider-->
          <HowItWorkSlider></HowItWorkSlider>
      </div><!-- end container -->
  </section><!-- end how-it-work-section -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'HowItWorkFour',
  props: ['classname', 'title', 'classnameTwo', 'subtitle', 'gutterBottom'],
  data () {
    return {
      SectionData
    }
  }
}
</script>
