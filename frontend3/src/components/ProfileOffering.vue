<template>
  <ProductsContainer 
    :products="products" 
    :loading="loading" 
    :hasMore="hasMore"
    @more="more()"/>
</template>

<script>
export default {
  name: 'ProfileOffering',
  props: {
    address: {
      type: String,
      required: true,
    },
  },

  data () {
    return {
      loading: false,
      hasMore: true,
      products: []
    }
  },

  methods: {
    more() {
      this.loading = true;
    }
  }
}
</script>