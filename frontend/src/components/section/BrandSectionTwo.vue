<template>
<section class="brand-section bg-gray py-5">
    <div class="container">
        <div class="row g-gs align-items-center">
            <div class="col-lg-4">
                <div class="brand-heading">
                    <h3>Loved by community & partners</h3>
                </div>
            </div><!-- end col -->
            <div class="col-lg-8">
                <div class="row g-gs">
                    <div class="col-6 col-md-3 col-sm-4" v-for="item in showItem" :key="item.id">
                        <div class="client-logo-item text-center" >
                            <img :src="item.img" alt="" class="img-fluid">
                        </div>
                    </div><!-- end col -->
                </div><!-- end row -->
            </div><!-- end col -->
        </div><!-- end row -->
    </div><!-- end container -->
</section><!-- end brand-section -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'BrandSectionTwo',
  data () {
    return {
      SectionData
    }
  },
  computed: {
    showItem() {
        return this.SectionData.brandData.brandList.filter(item => item.itemToShow === 'show')
    }
  }
}
</script>
