<template>
    <ul class="menu-list ms-lg-auto">
        <li class="menu-item">
          <router-link class="menu-link" :to="'/explore'">Explore</router-link>
        </li>
        <li v-if="IsConnected()" class="menu-item">
          <router-link class="menu-link" :to="'/me/owned'">Profile</router-link>
        </li>
     </ul>
</template>

<script>
import walletController from "@/mixins/walletController.js"

export default {
  name: 'MenuList',
  mixins:[walletController],

}
</script>
