<template>
  <div>
    <div v-if="!initialized">Initializing...</div>
      <div v-else>
        <ConnectSample />
        <!-- <QuerySample />
        <TxSample /> -->
      </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { initController, getController } from '../controller';

export default Vue.extend({
  name: 'IndexPage',
  data() {
    return {
      initialized: false
    }
  },

  beforeMount() {
    initController().then(() => {
      console.log("here");
      console.log(getController());
      
      this.initialized = true;
    });
  },
})
</script>
