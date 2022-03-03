<template>
  <section class="section-space-b how-it-work-section pt-2">
      <div class="container">
          <div class="row g-gs">
              <div class="col-sm-6 col-lg-3" v-for="item in SectionData.howItWorkData.howItWorkListTwo" :key="item.id">
                  <div class="card-hiw text-center card-hiw-s2">
                      <span class="card-hiw-number">{{ item.id }}</span>
                      <span :class="item.icon"></span>
                      <h5>{{ item.title }}</h5>
                  </div>
              </div><!-- end col -->
          </div><!-- end row -->
      </div><!-- end container -->
  </section><!-- end how-it-work-section -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'HowItWorkThree',
  data () {
    return {
      SectionData
    }
  }
}
</script>
