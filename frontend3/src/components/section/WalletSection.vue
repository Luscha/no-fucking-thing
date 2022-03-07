<template>
<section class="wallet-section section-space-b">
      <div class="container">
          <div v-if="!IsConnected()" class="row g-gs">
              <div class="col-sm-6 col-md-4 col-xl-4" v-for="item in GetAvailableConnections" :key="item.type">
                  <div class="card-media card-full card-media-s1 flex-column justify-content-center flex-wrap p-4">
                      <div class="d-flex flex-column align-items-center justify-content-center py-1">
                          <img :src="item.icon" alt="logo" class="card-media-img flex-shrink-0 me-0 mb-3">
                          <h6>{{ item.name }}</h6>
                      </div>
                      <div class="card-media-body flex-grow-0 mt-3">
                          <button v-on:click="connectAndRedirect(item.type, item.identifier)" class="btn btn-sm btn-outline-secondary">Connect</button>
                      </div>
                  </div><!-- end card-media -->
              </div>
              <div class="col-sm-6 col-md-4 col-xl-4" v-for="item in GetAvailableInstallTypes" :key="item.type">
                  <div class="card-media card-full card-media-s1 flex-column justify-content-center flex-wrap p-4">
                      <div class="d-flex flex-column align-items-center justify-content-center py-1">
                          <img :src="item.icon" alt="logo" class="card-media-img flex-shrink-0 me-0 mb-3">
                          <h6>{{ item.name }}</h6>
                      </div>
                      <div class="card-media-body flex-grow-0 mt-3">
                          <button v-on:click="this.Install(item.type)" class="btn btn-sm btn-outline-secondary">Install</button>
                      </div>
                  </div><!-- end card-media -->
              </div>
          </div>
          <div v-else>
            <button class="btn btn-dark" v-on:click="Disconnect()">Disconnect</button>
          </div>
      </div><!-- .container -->
  </section><!-- end wallet-section -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import walletController from '../../mixins/walletController.js'

export default {
  name: 'WalletSection',
  mixins: [walletController],
  data () {
    return {
      SectionData,
    }
  },

  methods: {
      connectAndRedirect(type, identifier) {
        this.Connect(type, identifier)
      }
  }
}
</script>
