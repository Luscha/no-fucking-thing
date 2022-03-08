<template>
  <div class="row g-gs">
    <div class="col-xl-3 col-lg-4 col-sm-6" v-for="product in products" :key="product.id+product.contractAddr">
        <Products :product="product"></Products>
    </div>
  </div>
</template>

<script>
import SectionData from '@/store/store.js'
import Products from '@/components/section/Products'
 
export default {
  name: 'ProductsContainer',
  components: { 
    Products 
  },
  props: {
    products: {
      type: Array,
      default() {
        return SectionData
      },
    },
  },
};
</script>