<template>
  <div>
    <nav class="header-menu menu nav">
        <!-- menu list -->
        <MenuList></MenuList>
        <!-- header btn -->
        <ul class="menu-btns">
            <li>
              <ButtonLink v-if="!IsConnected()" :text="'Connect'" link="/wallet" classname="btn" :class="classname"></ButtonLink>
              <a v-else @click.prevent="() => showAddressModal()" :class="`btn ${classname}`">{{ trunc(ConnectedAddress, 18) }}</a>
            </li>
            <li>
              <ThemeSwitcher></ThemeSwitcher>
            </li>
        </ul>
    </nav><!-- .header-menu -->

    <div class="modal fade" id="addressModal" tabindex="-1" aria-hidden="true">
        <div class="modal-dialog modal-dialog-centered">
            <div class="modal-content">
                <div class="modal-header">
                    <h4 class="modal-title">Your wallet</h4>
                    <button type="button" class="btn-close icon-btn" data-bs-dismiss="modal" aria-label="Close">
                        <em class="ni ni-cross"></em>
                    </button>
                </div><!-- end modal-header -->
                <div class="modal-body">
                    <a :href="terrascopeURL" target="_blank">{{ConnectedAddress}}</a>
                    <button v-on:click="() => Disconnect()" class="btn btn-primary d-block mt-3">Disconnect</button>
                </div><!-- end modal-body -->
            </div><!-- end modal-content -->
        </div><!-- end modal-dialog -->
    </div><!-- end modal-->
  </div>
</template>

<script>

// @ is an alias to /src
import MenuList from '@/components/common/MenuList.vue'
import walletController from "@/mixins/walletController.js"
import { trunc } from "@/utils/address";

import { Modal } from 'bootstrap';

export default {
  name: 'Menu',
  props: ['classname'],
  mixins:[walletController],
  components: {
    MenuList
  },

  data() {
    return {
      modal: undefined,
    }
  },

  computed: {
    terrascopeURL() {
      return `https://terrasco.pe/columbus-5/address/${this.ConnectedAddress}`
    }
  },

  methods: {
    trunc(str, len) {
      return trunc(str, len)
    },

    showAddressModal() {
      if (!this.modal) {
          this.modal = new Modal(document.getElementById('addressModal'));
      }
      this.modal.show()
    },
  }
}
</script>

<style>
/*TODO FIX ME*/
.modal-backdrop {
  z-index: 4;
}
</style>