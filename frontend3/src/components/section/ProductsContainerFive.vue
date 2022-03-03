<template>
  <div class="row g-gs">
    <div class="col-xl-3 col-lg-4 col-sm-6" v-for="product in SectionData.productData.products" :key="product.id">
        <ProductsFive :product="product"></ProductsFive>
    </div>
  </div>
</template>

<script>
import SectionData from '@/store/store.js'
import ProductsFive from '@/components/section/ProductsFive'

export default {
  name: 'ProductsContainerFive',
  components: { 
    ProductsFive 
  },
  data () {
      return {
        SectionData,
      }
  }

};
</script>