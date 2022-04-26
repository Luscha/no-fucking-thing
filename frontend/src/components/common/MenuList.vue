<template>
    <ul class="menu-list ms-lg-auto">
        <li class="menu-item">
          <router-link class="menu-link" :to="'/explore'">Explore</router-link>
        </li>
        <li class="menu-item">
          <router-link class="menu-link" :to="'/me/owned'">Profile</router-link>
        </li>
     </ul>
</template>

<script>
export default {
  name: 'MenuList',
}
</script>
