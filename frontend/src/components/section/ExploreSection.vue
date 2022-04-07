<template>
    <section class="explore-section pt-4 pb-4">
        <div class="container">
            <div class="explore-items-wrap">
                <ul class="nav nav-tabs nav-tabs-s1" id="myTab" role="tablist">
                    <li class="nav-item" role="presentation" v-for="list in sections" :key="list.id">
                        <button class="nav-link" 
                          :class="list.isActive" 
                          :id="list.slug" 
                          data-bs-toggle="tab" 
                          :data-bs-target="list.bsTarget" 
                          type="button" 
                          v-on:click="() => {tab = list.id}">{{ list.title }}</button>
                    </li>
                </ul>
                <div class="gap-2x"></div><!-- end gap-2x -->
                <div class="tab-content" id="myTabContent">
                    <div class="tab-pane fade show active" id="on-sale" role="tabpanel" aria-labelledby="on-sale-tab">
                        <ProductsContainer :products="offerings"></ProductsContainer>
                    </div><!-- end tab-pane -->
                    <div class="tab-pane fade" id="collections" role="tabpanel" aria-labelledby="collections-tab">

                    </div><!-- end tab-pane -->
                </div><!-- end tab-content -->
            </div><!-- end explore-items-wrap -->
        </div><!-- .container -->
    </section><!-- end explore-section -->
</template>

<script>
import {parseOffering} from '@/utils/nft-offer'
import * as query from '@/contract/query'
import { MARKETPLACE_ADDRESS } from '@/config'

const LOAD_CHUNK = 20;

export default {
  name: 'ExploreSection',

  data() {
    return {
      loading: false,
      tab: 1,
      sections: [
        {
          id: 1,
          isActive: 'active',
          title: 'On Sale',
          slug: 'on-sale-tab',
          bsTarget: '#on-sale',
          more: true,
        },
        {
          id: 2,
          title: 'Collections',
          slug: 'collections-tab',
          bsTarget: '#collections',
          more: true,
        },
      ],
      offerings: []
    }
  },

  mounted() {
    this.loading = true;
    query.query(MARKETPLACE_ADDRESS, { offerings: { limit: LOAD_CHUNK } })
    .then(res => {
      this.offerings = res.offerings.map(nft => (parseOffering(nft)));
      this.sections[this.tab-1].more = this.offerings.length == LOAD_CHUNK
    })
    .finally(() => this.loading = false);

    window.onscroll = () => {
        if (this.loading) {
          return;
        }

        let bottomOfWindow = document.documentElement.scrollTop + window.innerHeight > document.documentElement.offsetHeight - 200;
        if (this.sections[this.tab-1].more && bottomOfWindow) {
          this.loading = true;
          if (this.tab == 1) {
            query.query(MARKETPLACE_ADDRESS, { offerings: { start_after: this.offerings[this.offerings.length-1].id, limit: LOAD_CHUNK } })
            .then(res => {
              this.offerings = this.offerings.concat(res.offerings.map(nft => (parseOffering(nft))));
              this.sections[this.tab-1].more = res.offerings == LOAD_CHUNK
            })
            .finally(() => this.loading = false);
          }
        }
      }
  }
}
</script>

<style lang="scss" scoped>
 .explore-items-wrap {
   .nav {
      display: flex;
      justify-content: center;
   }
 }
</style>