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

<script>
import { initController, getController } from '../../controller';
import ConnectSample from '@/components/ConnectSample.vue';

export default {
  name: 'IndexPage',
  components: {
    ConnectSample
  },
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
}
</script>
