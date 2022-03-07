<template>
  <nav class="header-menu menu nav">
      <!-- menu list -->
      <!-- <MenuList></MenuList> -->
      <!-- header btn -->
      <ul class="menu-btns">
          <li><ButtonLink :text="IsConnected() ? trunc(GetConnectedAddress(), 18) : 'Connect'" link="/wallet" classname="btn" :class="classname"></ButtonLink></li>
          <li>
             <ThemeSwitcher></ThemeSwitcher>
          </li>
      </ul>
  </nav><!-- .header-menu -->
</template>

<script>

// @ is an alias to /src
// import MenuList from '@/components/common/MenuList.vue'
import walletController from "@/mixins/walletController.js"

export default {
  name: 'Menu',
  props: ['classname'],
  mixins:[walletController],
  // components: {
  //   MenuList
  // },

  methods: {
    trunc(str, len) {
      const min = 8+3+3;
      if (str.length <= len || len < min) {
        return str;
      }
      return str.substring(0, 8) + "..." + str.substring(str.length-(len-8-3));
    }
  }
}
</script>
