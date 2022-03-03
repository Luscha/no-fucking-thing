<template>
    <div class="hero-slider-wrap">
        <div class="container">
            <!-- Featured Item Slider -->
            <FullScreenSlider></FullScreenSlider>
        </div><!-- .container-->
    </div><!-- end hero-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'HeroEight',
  data () {
    return {
      SectionData
    }
  }
}
</script>
