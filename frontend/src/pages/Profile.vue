<template>
  <div class="page-wrap">
        <!-- header  -->
        <ProfileHeader :address="address" :me="false"/>
        <!-- Author section  -->
        <AuthorSection :key="address" :address="address" :me="me"></AuthorSection>
        <!-- Footer  -->
        <Footer classname="bg-dark on-dark"></Footer>
  </div><!-- end page-wrap -->
</template>

<script>
import ProfileHeader from '@/components/ProfileHeader.vue';

export default {
  name: 'Profile',
  components: {
    ProfileHeader
  },

  data() {
    return {
      address: this.$route.params.address,
    }
  },
}
</script>
