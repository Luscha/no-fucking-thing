<template>
  <div>
    <!-- header  -->
    <ProfileHeader :address="address"/>
    <section class="user-panel-section section-space">
      <div class="container">
        <div class="row">
          <UserSidebar :title="sectionInfo.title" :lists="sections" :active="section"></UserSidebar>

          <div class="col-lg-8">
            <div class="user-panel-title-box">
              <h3>{{ sectionInfo.title }}</h3>
            </div><!-- end user-panel-title-box -->

            <ProfileOwned v-if="section == 'owned'" :address="address"/>

            <ProfileOffering v-if="section == 'on-sale'" :address="address"/>

          </div><!-- end col-lg-8 -->
        </div><!-- end row -->
    </div><!-- end container -->
    </section><!-- end user-panel-section -->
    <!-- Footer  -->
    <Footer classname="bg-dark on-dark"></Footer>
  </div><!-- end page-wrap -->
</template>

<script>
import ProfileHeader from '@/components/ProfileHeader.vue';
import ProfileOwned from '@/components/ProfileOwned.vue';
import ProfileOffering from '@/components/ProfileOffering.vue';

import walletController from '@/mixins/walletController.js';

export default {
  name: 'ProfileMe',

  components: {
    ProfileHeader,
    ProfileOwned,
    ProfileOffering
  },

  mixins: [walletController],

  data() {
    return {
      section: this.$route.params.section,
      sections: [
        {
          id: "owned",
          // icon: 'ni-gift',
          title: 'Owned',
          path: '/me/owned'
        },
        {
          id: "on-sale",
          // icon: 'ni-gift',
          title: 'On Sale',
          path: '/me/on-sale'
        },
        // {
        //   id: "collections",
        //   // icon: 'ni-gift',
        //   title: 'Collections',
        //   path: '/me/collections'
        // },
      ]
    }
  },

  computed: {
    address() {
      return this.ConnectedAddress || ""
    }, 
    sectionInfo() {
      return this.sections.find(s => s.id == this.section);
    }
  },

  // mounted() {
  //   if (!this.IsConnected()) {
  //     this.$router.push("/")
  //     return;
  //   }
  // },
}
</script>
