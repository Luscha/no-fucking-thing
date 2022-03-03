<template>
  <div class="row g-gs">
    <div class="col-xl-3 col-lg-4 col-sm-6" v-for="product in SectionData.productData.products" :key="product.id">
        <ProductsThree :product="product"></ProductsThree>
    </div>
  </div>
</template>

<script>
import SectionData from '@/store/store.js'
import ProductsThree from '@/components/section/ProductsThree'

export default {
  name: 'ProductsContainerThree',
  components: { 
    ProductsThree 
  },
  data () {
      return {
        SectionData,
      }
  }

};
</script>