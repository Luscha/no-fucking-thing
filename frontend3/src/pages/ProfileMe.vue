<template>
  <div>
    <!-- header  -->
    <ProfileHeader :address="address"/>
    <section class="user-panel-section section-space">
      <div class="container">
        <div class="row">
          <UserSidebar :title="'a'" :lists="sections"></UserSidebar>
          <div class="col-lg-8">
        <div class="user-panel-title-box">
            <h3>asda sda</h3>
        </div><!-- end user-panel-title-box -->
        <div class="profile-setting-panel-wrap">
            <p class="fs-14 mb-4">sdf sdf dsf sdf</p>
            <h5 class="mb-3"> dsf dsf dsf </h5>
            <form action="#">
                <div class="row g-4">
                    <div class="col-md-4 col-sm-6">
                        <input type="text" class="form-control form-control-s1" placeholder="Display code">
                    </div>
                    <div class="col-md-4 col-sm-6">
                        <button class="btn btn-dark">sdf dsf ds fsd</button>
                    </div>
                </div>
            </form>
            <hr class="my-4">
            <h5 class="mb-3">sdf dsf ds</h5>
            <a href="#" class="btn btn-outline-dark">sdf dsf ds</a>
            <p class="fs-14 mt-2">sdf dsf s</p>
        </div><!-- end profile-setting-panel-wrap-->
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

import walletController from '@/mixins/walletController.js';

export default {
  name: 'ProfileMe',

  components: {
    ProfileHeader
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
          path: 'me/owned'
        },
        {
          id: "on-sale",
          // icon: 'ni-gift',
          title: 'On Sale',
          path: 'me/on-sale'
        },
        {
          id: "collections",
          // icon: 'ni-gift',
          title: 'Collections',
          path: 'me/collections'
        },
      ]
    }
  },

  computed: {
    address() {
      return this.ConnectedAddress || ""
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
