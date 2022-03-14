<template>
  <nav class="header-menu menu nav">
      <!-- menu list -->
      <MenuList></MenuList>
      <!-- header btn -->
      <ul class="menu-btns">
          <li>
            <ButtonLink v-if="!IsConnected()" :text="'Connect'" link="/wallet" classname="btn" :class="classname"></ButtonLink>
            <ButtonLink v-else :text="trunc(ConnectedAddress, 18)" :link="'/me/owned'" classname="btn" :class="classname"></ButtonLink>
          </li>
          <li>
             <ThemeSwitcher></ThemeSwitcher>
          </li>
      </ul>
  </nav><!-- .header-menu -->
</template>

<script>

// @ is an alias to /src
import MenuList from '@/components/common/MenuList.vue'
import walletController from "@/mixins/walletController.js"
import { trunc } from "@/utils/address";

export default {
  name: 'Menu',
  props: ['classname'],
  mixins:[walletController],
  components: {
    MenuList
  },

  methods: {
    trunc(str, len) {
      return trunc(str, len)
    }
  }
}
</script>
