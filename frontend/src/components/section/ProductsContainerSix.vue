<template>
  <div class="row g-gs">
    <div class="col-xl-3 col-lg-4 col-sm-6" v-for="product in SectionData.productData.products" :key="product.id">
        <ProductsSix :product="product"></ProductsSix>
    </div>
  </div>
</template>

<script>
import SectionData from '@/store/store.js'
import ProductsSix from '@/components/section/ProductsSix'
 
export default {
  name: 'ProductsContainerSix',
  components: { 
    ProductsSix 
  },
  data () {
      return {
        SectionData,
      }
  }

};
</script>