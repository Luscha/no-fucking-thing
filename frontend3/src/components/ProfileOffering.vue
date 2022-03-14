<template>
  <ProductsContainer 
    :products="offerings" 
    :loading="loading" 
    :hasMore="hasMore"
    :fullScreen="!me"
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
    me: {
      type: Boolean,
      default: true,
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
        this.loadOfferings(address);
      }
    }
  },

  methods: {
    withdraw(id) {
      this.offerings = this.offerings.filter(p => p.id != id)
    },

    more() {
      this.loading = true;
      this.loadOfferings(this.address, this.offerings[this.offerings.length-1].id);
    }
  },

  mounted() {
    this.loading = true;
    this.loadOfferings(this.address);
  }
}
</script>