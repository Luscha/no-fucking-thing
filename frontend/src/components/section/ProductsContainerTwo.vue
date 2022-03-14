<template>
    <div class="row g-gs">
        <div class="col-lg-6" v-for="product in showItem" :key="product.id">
             <ProductsTwo :product="product"></ProductsTwo>
        </div><!-- end col -->
    </div><!-- end row -->
</template>
<script>

// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import ProductsTwo from '@/components/section/ProductsTwo'

export default {
  name: 'ProductsContainerTwo',
  components: { 
    ProductsTwo
  },
  data () {
    return {
      SectionData
    }
  },
  computed: {
    showItem() {
        return this.SectionData.productData.products.filter(item => item.itemToShow === 'show')
    }
  }
}
</script>