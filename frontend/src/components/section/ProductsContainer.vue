<template>
  <div class="row g-gs">
    <div :class="{
      'col-xl-3': fullScreen,
      'col-lg-4': fullScreen,
      'col-xl-6': !fullScreen,
      'col-lg-6': !fullScreen,
      }" class="col-sm-6" v-for="product in products" :key="product.id+product.contractAddr">
        <Products :product="product" @withdraw="$emit('withdraw', $event)"></Products>
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

    hasMore: {
      type: Boolean,
      default: true
    },

    loading: {
      type: Boolean,
      default: false
    },

    fullScreen: {
      type: Boolean,
      default: true
    }
  },

  mounted() {
    window.onscroll = () => {
      if (this.loading) {
        return;
      }

      let bottomOfWindow = document.documentElement.scrollTop + window.innerHeight > document.documentElement.offsetHeight - 200;
      if (this.hasMore && bottomOfWindow) {
        this.$emit('more')
      }
    }
  }
};
</script>