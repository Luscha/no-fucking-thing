<template>
    <div>
      <div v-if="me" class="row g-4">
        <p class="col-md-6 col-sm-12 smaller mb-2">Not finding your NFT's? Try to add the contract address to our database in order to find information about them.</p>
        <div class="col-md-6 col-sm-12">
          <div :style="{display: 'flex'}">
            <input v-model="importAddress" type="text" class="form-control form-control-s1" placeholder="NFT contract address">
            <button @click="importCollection" class="btn btn-dark">Import</button>
          </div>
        </div>
      </div>
      <hr v-if="me" class="my-4">
      <NftsContainer
        :nfts="owned" 
        :loading="loading" 
        :hasMore="hasMore"
        :fullScreen="!me"
        @more="more()"/>
    </div><!-- row -->
</template>

<script>
import token from '@/mixins/token-owned';
import { signMessage } from '@/utils/sign-message';
import walletController from "@/mixins/walletController.js"
import axios from 'axios';
import { BACKEND_REST_API_ENDPOINT } from "@/config"

export default {
  name: 'ProfileOwned',
  mixins: [token, walletController],

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
      importAddress: ""
    }
  },

  watch: {
    'address': {
      handler(address) {
        this.clear(),
        this.loadOwned(address);
      }
    }
  },

  methods: {
    more() {
      this.loading = true;
      this.loadOwned(this.address);
    },

    async importCollection() {
      if (this.importAddress == "") {
        return;
      }

      const wallet = this.GetWallet();
      if (!wallet) {
          return;
      }   

      const message = await signMessage(wallet, {contractAddrs: this.importAddress})

      const res = await axios.post(BACKEND_REST_API_ENDPOINT + "/collections", message);

      await this.loadContracts();
      this.loadOwned(this.address);
    }
  },

  async mounted() {
    this.loading = true;
    await this.loadContracts();
    this.loadOwned(this.address);
  }
}
</script>