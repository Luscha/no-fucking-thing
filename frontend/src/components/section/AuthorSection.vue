<template>
<section class="author-section section-space">
    <div class="container">
        <div class="row">
            <div class="col-xl-12 ps-xl-4">
                <div class="author-items-wrap">
                    <ul class="nav nav-tabs nav-tabs-s1" id="myTab" role="tablist">
                        <li class="nav-item" role="presentation" v-for="list in sections" :key="list.id">
                            <button class="nav-link" 
                                :class="list.isActive" 
                                :id="list.slug" 
                                data-bs-toggle="tab" 
                                :data-bs-target="list.bsTarget"
                                v-on:click="() => {tab = list.id}" 
                                type="button">{{ list.title }}</button>
                        </li>
                    </ul>
                    <div class="gap-2x"></div><!-- end gap-2x -->
                    <div class="tab-content" id="myTabContent">
                        <div class="tab-pane fade show active" id="owned" role="tabpanel" aria-labelledby="owned-tab">
                            <ProfileOwned :address="address" :me="false" />
                        </div><!-- end tab-pane -->
                        <div class="tab-pane fade" id="on-sale" role="tabpanel" aria-labelledby="on-sale-tab">
                            <ProfileOffering :address="address" :me="false" />
                        </div><!-- end tab-pane -->
                    </div><!-- end tab-content -->
                </div><!-- end author-items-wrap -->
            </div><!-- end col -->
        </div><!-- end row -->
    </div><!-- .container -->
</section><!-- end author-section -->
</template>

<script>

import ProfileOwned from '@/components/ProfileOwned.vue';
import ProfileOffering from '@/components/ProfileOffering.vue';

export default {
  name: 'AuthorSection',

components: {
    ProfileOwned,
    ProfileOffering
  },

  props: {
    address: {
        type: String,
        required: true,
    },
  },

  data () {
    return {
      tab: 1,
      sections: [
        {
          id: 1,
          isActive: 'active',
          title: 'Owned',
          slug: 'owned-tab',
          bsTarget: '#owned',
          more: true,
        },
        {
          id: 2,
          title: 'On Sale',
          slug: 'on-sale-tab',
          bsTarget: '#on-sale',
          more: true,
        },
      ],
    }
  },
}
</script>

<style lang="css" scoped>
 .details {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 1;
 }
 .author-link,
 .card-price-wrap {
   z-index: 2;
   position: relative;
 }
</style>