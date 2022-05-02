<template>
  <div class="page-wrap">
        <!-- header  -->
        <header class="header-section has-header-main bg-gradient">
          <HeaderMain></HeaderMain>
          <!-- hero -->
          <div class="hero-wrap">
            <div class="container">
                <div class="row align-items-center flex-md-row-reverse">
                    <div class="col-lg-6 col-sm-9 col-md-6">
                        <div class="hero-image hero-image-mobile">
                            <img :src="data.image" alt="" class="w-100">
                        </div>
                    </div>
                    <div class="col-lg-6 col-md-6">
                        <div class="hero-content">
                            <h5 class="mb-3 text-uppercase hero-text">{{ data.subtitle }}</h5>
                            <h1 class="hero-title mb-4">{{ data.title }}</h1>
                            <div class="d-flex align-items-center mb-4">
                                <h2 class="fs-7 me-3">{{ gen0Info.humanPrice }}</h2>
                                <span class="fw-semibold">{{ data.remaining }} {{available}}</span>
                            </div>
                              <a v-if="gen0Info.max_tokens > gen0Info.issued && IsConnected()" href="#" 
                                @click.prevent="() => showMintModal()" class="btn btn-dark d-block">Mint!</a>
                        </div><!-- hero-content -->
                    </div><!-- col-lg-6 -->
                </div>
            </div><!-- .container-->
          </div><!-- end hero-wrap -->
        </header>
        <!-- Footer  -->
        <Footer classname="bg-dark on-dark"></Footer>

        <div class="modal fade" id="mintModal" tabindex="-1" aria-hidden="true">
          <div class="modal-dialog modal-dialog-centered">
              <div class="modal-content">
                  <div class="modal-header">
                      <h4 class="modal-title">Sell Nft</h4>
                      <button type="button" class="btn-close icon-btn" data-bs-dismiss="modal" aria-label="Close">
                          <em class="ni ni-cross"></em>
                      </button>
                  </div><!-- end modal-header -->
                  <div class="modal-body">
                      <p class="mb-3">
                          By minting a <strong>NoFuckingThing Gen0 NFT</strong> you will support the project and become part of the governance 
                      </p>
                      <p class="mb-3 text-red .smaller lh">
                        The cost is just <strong class="text-red">{{gen0Info.humanPrice}}</strong>
                      </p>
                      <button v-on:click="() => mint()" class="btn btn-primary d-block">Mint</button>
                  </div><!-- end modal-body -->
              </div><!-- end modal-content -->
          </div><!-- end modal-dialog -->
      </div><!-- end modal-->
  </div><!-- end page-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import * as query from '@/contract/query'
import { GEN0_ADDRESS } from '@/config'

import walletController from "@/mixins/walletController.js"
import { exec } from '@/contract/execute';

import { Modal } from 'bootstrap';

export default {
  name: 'MintGen0',
  mixins: [walletController],

  data () {
    return {
      data: {
        image: "https://pbs.twimg.com/media/FPBzhwtXoAMJ7Pl?format=jpg&name=4096x4096",
        subtitle: "Support NoFancyThings and get future rewards",
        title: "NoFancyThings Gen0 is out and is limited!",
        remaining: "Available Gen0 NFT:"
      },
      gen0Info: {},
      modal: undefined,
      SectionData
    }
  },
  computed: {
    available() {
      return `${this.gen0Info.max_tokens-this.gen0Info.issued}/${this.gen0Info.max_tokens}`
    }
  },

  mounted() {
    this.load();
  },

  methods: {
    load() {
      const denom = {
        uluna: "LUNA",
        uusd: "UST"
      }
      query.query(GEN0_ADDRESS, { contract_info: { } }).then(res => {
        this.gen0Info = {...this.gen0Info, ...res};
        this.gen0Info.humanPrice = `${res.minting_price_amount/1000000} ${denom[res.minting_price_denom]}`
      });
      query.query(GEN0_ADDRESS, { num_tokens: { } }).then(res => {
        this.gen0Info.issued = res.count;
      });
    },

    showMintModal() {
      if (!this.modal) {
          this.modal = new Modal(document.getElementById('mintModal'));
      }
      this.modal.show()
    },

    mint() {
      const wallet = this.GetWallet();
      if (!wallet) {
          return;
      }           

      const coins = {};
      coins[this.gen0Info.minting_price_denom] = this.gen0Info.minting_price_amount;

      exec(wallet, GEN0_ADDRESS, 
          { mint: {
              owner: this.ConnectedAddress,
              payment: {
                denom: this.gen0Info.minting_price_denom,
                amount: this.gen0Info.minting_price_amount.toString()
              }
            }
          },
          coins,
          )
          .then(() => {
              this.$notify({
                  title: 'Success!!',
                  text: 'You successfully minted a new Gen0 NFT',
                  type: 'success',
              });
              this.load();
          })
          .catch(err => {
              console.log(err);
              this.$notify({
                  title: 'Error occurred',
                  text: 'Try to refresh the page and purchase again',
                  type: 'error',
              });
          })
          .finally(() => {
            this.modal.hide();
          })
          
      this.$notify({
          title: 'Processing transaction',
          text: 'Wait some time on this page',
          duration: 20000
      });
    },
  }
}
</script>