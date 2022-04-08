<template>
    <!-- header  -->
    <header class="header-section has-header-main bg-pattern-3">
        <!-- Header main -->
        <HeaderMain></HeaderMain>
        <!-- hero -->
        <div class="hero-wrap sub-header" :style="{ bannerbackgroundImage: banner ? 'url(' + coverimg + ')' : undefined }">
        <div class="overlay"></div>
            <div class="container">
                <div class="hero-content py-0 d-flex align-items-center">
                    <div v-if="avatar" class="avatar flex-shrink-0" :class="'avatar-3'">
                    <img :src="img" alt="">
                    </div>
                    <div class="author-hero-content-wrap d-flex flex-wrap justify-content-between ms-3 flex-grow-1">
                        <div class="author-hero-content me-3">
                            <h4 class="hero-author-title mb-1 text-white">Profile of</h4>
                            <!-- <p class="hero-author-username mb-1 text-white">{{ username }}</p> -->
                            <div class="d-flex align-items-center">
                                <input type="text" :size="address.length+2" :style="{maxWidth: 'unset'}" class="copy-input text-white" :value="address" id="copy-input" readonly>
                                <div class="tooltip-s1">
                                    <button v-clipboard:copy="address" v-clipboard:success="onCopy"  class="copy-text text-white ms-2" type="button">
                                        <span class="tooltip-s1-text tooltip-text">Copy</span>
                                        <em class="ni ni-copy"></em>
                                    </button>
                                </div>
                            </div>
                        </div><!-- author-hero-conetent -->
                        <div class="hero-action-wrap d-flex align-items-center my-2">
                            <div v-if="me">
                            <button class="btn btn-dark" v-on:click="Disconnect()">Disconnect</button>
                            </div>
                            <!-- <router-link v-if="me" :to="'/account'" class="btn btn-light">Edit Profile</router-link> -->
                            <!-- <div v-else: class="dropdown ms-3">
                                <a class="icon-btn icon-btn-s1" href="#" data-bs-toggle="dropdown" id="reportDropdown">
                                    <em class="ni ni-more-h"></em>
                                </a>
                                <ul class="dropdown-menu card-generic p-2 dropdown-menu-end mt-2 card-generic-sm" aria-labelledby="reportDropdown">
                                    <li><a class="dropdown-item card-generic-item" href="#" data-bs-toggle="modal" data-bs-target="#reportModal">Report Page</a></li>
                                </ul>
                            </div> -->
                            <!--end dropdown -->
                        </div><!--end hero-action-wrap -->
                    </div><!-- author-hero-content -->
                </div><!-- hero-content -->
            </div><!-- .container-->
        </div><!-- end hero-wrap -->
    </header>
</template>

<script>

import walletController from "@/mixins/walletController.js"

export default {
  name: 'ProfileHeader',
  mixins:[walletController],
  
  props: {
      me: {
        type: Boolean,
        default: true,
      },
      address: {
        type: String,
        required: true,
      }
  },

  // TODO load banner, cover, name
  data() {
    return {
      avatar: undefined,
      banner: undefined,
    }
  },

  setup() {
    const onCopy = (e) => {
      let target = e.trigger.querySelector(".tooltip-text");
      let prevText = target.innerHTML;
      target.innerHTML = "Copied";
      setTimeout(function(){
        target.innerHTML = prevText;
      }, 1000)
    }
    return { onCopy }
  }
}
</script>