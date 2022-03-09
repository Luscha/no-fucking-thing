<template>
  <div>
    <router-view :key="$route.path"/>
    <notifications position="top right"/>
  </div>
</template>

<script>
// import { computed, reactive } from 'vue'
import { useHead } from '@vueuse/head'

export default {
  name: 'App',

  setup() {
    useHead({
      meta: [
        {
          name: "terra-wallet"
        }
      ],
    })
  },
}
</script>
