<template>
  <ProductsContainer 
    :products="offerings" 
    :loading="loading" 
    :hasMore="hasMore"
    :fullScreen="false"
    @withdraw="withdraw"
    @more="more()"/>
</template>

<script>
import token from '@/mixins/token-offering';

export default {
  name: 'ProfileOffering',
  mixins: [token],

  props: {
    address: {
      type: String,
      required: true,
    },
  },

  data () {
    return {
    }
  },

  watch: {
    'address': {
      handler(address) {
        this.clear(),
        this.load(address);
      }
    }
  },

  methods: {
    withdraw(id) {
      this.offerings = this.offerings.filter(p => p.id != id)
    },

    more() {
      this.loading = true;
      this.load(this.address, this.offerings[this.offerings.length-1].id);
    }
  },

  mounted() {
    this.loading = true;
    this.load(this.address);
  }
}
</script>