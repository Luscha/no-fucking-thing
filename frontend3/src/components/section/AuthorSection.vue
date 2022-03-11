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
                            <div class="row g-gs">
                                <NftsContainer :nfts="owned" />
                            </div><!-- row -->
                        </div><!-- end tab-pane -->
                        <div class="tab-pane fade" id="on-sale" role="tabpanel" aria-labelledby="on-sale-tab">
                            <div class="row g-gs">
                                <ProductsContainer :products="offerings"></ProductsContainer>
                            </div><!-- row -->
                        </div><!-- end tab-pane -->
                    </div><!-- end tab-content -->
                </div><!-- end author-items-wrap -->
            </div><!-- end col -->
        </div><!-- end row -->
    </div><!-- .container -->
</section><!-- end author-section -->
</template>

<script>
import { parseOffering } from '@/utils/nft-offer'
import * as query from '@/contract/query'
// import { NftWrapper } from '@/models/nft-wrapper';

const LOAD_CHUNK = 20;

export default {
  name: 'AuthorSection',
  props: {
    address: {
        type: String,
        required: true,
    },
    me: {
        type: Boolean,
        required: true,
    }
  },

  data () {
    return {
      loading: false,
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
      offerings: [],
      owned: [],
      ownedIDs: [],
      collectionInfos: {},
    }
  },

  methods: {
    loadOfferings(start = undefined) {
        query.query("marketplace", { offerings_by_owner: { owner: this.address, limit: LOAD_CHUNK, start_after: start } })
        .then(res => {
            this.offerings = this.offerings.concat(res.offerings.map(nft => (parseOffering(nft))));
            this.sections[1].more = this.offerings.length == LOAD_CHUNK;
        })
        .finally(() => this.loading = false);
    },

    loadOwned(start = undefined) {
        // TODO get all contracts addresses
        query.query("gen0", { contract_info: {} })
        .then(res => {
            this.collectionInfos["gen0"] = res
        })
        .catch(err => console.log(err));

        query.query("gen0", { tokens: { owner: this.address, limit: LOAD_CHUNK, start_after: start } })
        .then(res => {
            this.ownedIDs = this.ownedIDs.concat(res.tokens);
            this.sections[0].more = res.tokens == LOAD_CHUNK;
            this.owned = this.owned.concat(res.tokens.map(res => ({contractAddr: "terra1fn9wsg32jpdlz0n84w42fdf593u0tjlkcmjvqa", token_id: res})));
        })
        .finally(() => this.loading = false);
    }
  },

  mounted() {
    console.log("mounting")
    this.loading = true;
    this.loadOfferings();
    this.loadOwned();

    window.onscroll = () => {
        if (this.loading) {
            return;
        }

        let bottomOfWindow = document.documentElement.scrollTop + window.innerHeight > document.documentElement.offsetHeight - 200;
        if (this.sections[this.tab-1].more && bottomOfWindow) {
            this.loading = true;
            if (this.tab == 2) {
                this.loadOfferings(this.offerings[this.offerings.length-1].id)
            } else if (this.tab == 1) {
                this.loadOwned(this.owned[this.owned.length-1].id)
            }
        }
    }
  }
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