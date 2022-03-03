<template>
  <div class="row g-gs">
    <div class="col-xl-3 col-lg-4 col-sm-6" v-for="product in SectionData.productData.products" :key="product.id">
        <ProductsFour :product="product"></ProductsFour>
    </div>
  </div>
</template>

<script>
import SectionData from '@/store/store.js'
import ProductsFour from '@/components/section/ProductsFour'

export default {
  name: 'ProductsContainerFour',
  components: { 
    ProductsFour 
  },
  data () {
      return {
        SectionData,
      }
  }

};
</script>