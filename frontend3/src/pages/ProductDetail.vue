<template>
<div class="page-wrap">
    <!-- header  -->
    <header class="header-section has-header-main">
        <!-- Header main -->
        <HeaderMain></HeaderMain>
    </header>
    <section class="item-detail-section section-space">
            <div class="container">
                <div class="row">
                    <div class="col-lg-6 pe-xl-5">
                        <div class="item-detail-content">
                            <div class="item-detail-img-container mb-4">
                                <img :src="nft.image" alt="" class="w-100 rounded-3">
                            </div><!-- end item-detail-img-container -->

                        </div><!-- end item-detail-content -->
                    </div><!-- end col -->

                    <!-- DETAILS-->
                    <div class="col-lg-6">
                        <div class="item-detail-content mt-4 mt-lg-0">
                            <h1 class="item-detail-title mb-2">{{ nft.name }}</h1>
                            <!-- <div class="item-detail-meta d-flex flex-wrap align-items-center mb-3">
                                <span class="item-detail-text-meta">{{ nft.name }}</span>
                                <span class="dot-separeted"></span>
                                <span class="item-detail-text-meta">{{ metaTextTwo }}</span>
                                <span class="dot-separeted"></span>
                                <span class="item-detail-text-meta" v-html="metaTextThree"></span>
                            </div> -->
                            <p class="item-detail-text mb-4">{{ nft.description }}</p>
                            <div class="item-credits">
                                <div class="row g-4">
                                    <div class="col-xl-6">
                                        <div class="card-media card-media-s1">
                                            <router-link v-if="minter.avatar" :to="'/author/'+minter.address" class="card-media-img flex-shrink-0 d-block">
                                                <img :src="minter.avatar" alt="avatar">
                                            </router-link>
                                            <div class="card-media-body">
                                                <router-link :to="'/author/'+minter.address" class="fw-semibold">{{ trunc(minter.address, 18) }}</router-link>
                                                <p class="fw-medium small">Minter</p>
                                            </div>
                                        </div><!-- end card -->
                                    </div><!-- end col-->
                                    <div class="col-xl-6">
                                        <div class="card-media card-media-s1">
                                            <router-link v-if="collection.avatar" :to="'/collections/'+contract" class="card-media-img flex-shrink-0 d-block">
                                                <img :src="collection.avatar" alt="avatar">
                                            </router-link>
                                            <div class="card-media-body">
                                                <router-link :to="'/collections/'+contract" class="fw-semibold">{{ collection.name }}</router-link>
                                                <p class="fw-medium small">Collection</p>
                                            </div>
                                        </div><!-- end card -->
                                    </div><!-- end col-->
                                    <div class="col-xl-12">
                                        <div class="card-media card-media-s1">
                                            <router-link v-if="minter.avatar" :to="'/author/'+owner.address" class="card-media-img flex-shrink-0 d-block">
                                                <img :src="item.avatar" alt="avatar">
                                            </router-link>
                                            <div class="card-media-body">
                                                <router-link :to="'/author/'+owner" class="fw-semibold">{{ owner.address }}</router-link>
                                                <p class="fw-medium small">Owner</p>
                                            </div>
                                        </div><!-- end card -->
                                    </div><!-- end col-->
                                </div><!-- end row -->
                            </div><!-- end row -->
                            <div class="item-detail-btns mt-4">
                                <ul class="btns-group d-flex">
                                    <li class="flex-grow-1">
                                        <a href="#" data-bs-toggle="modal" data-bs-target="#placeBidModal" class="btn btn-dark d-block">{{ SectionData.itemDetailData.btnText }}</a>
                                    </li>
                                    <li class="flex-grow-1">
                                        <div class="dropdown">
                                            <a href="#" class="btn bg-dark-dim d-block" data-bs-toggle="dropdown">{{ SectionData.itemDetailData.btnTextTwo }}</a>
                                            <div class="dropdown-menu card-generic p-2 keep-open w-100 mt-1">
                                                <router-link :to="icon.path" class="dropdown-item card-generic-item" v-for="(icon, i) in SectionData.socialShareList" :key="i"><em class="ni me-2" :class="icon.btnClass"></em>{{ icon.title }}</router-link>
                                            </div>
                                        </div>
                                    </li>
                                </ul>
                            </div><!-- end item-detail-btns -->
                        </div><!-- end item-detail-content -->
                    </div><!-- end col -->
                </div><!-- end row -->
            </div><!-- .container -->
    </section><!-- end item-detail-section -->
    <!-- Related product -->
    <!-- <RelatedProduct></RelatedProduct> -->
    <!-- Footer  -->
    <Footer classname="bg-dark on-dark"></Footer>
     <!-- Modal -->
    <div class="modal fade" id="placeBidModal" tabindex="-1" aria-hidden="true">
        <div class="modal-dialog modal-dialog-centered">
            <div class="modal-content">
                <div class="modal-header">
                    <h4 class="modal-title">{{ SectionData.placeBidModal.title }}</h4>
                    <button type="button" class="btn-close icon-btn" data-bs-dismiss="modal" aria-label="Close">
                        <em class="ni ni-cross"></em>
                    </button>
                </div><!-- end modal-header -->
                <div class="modal-body">
                    <p class="mb-3" v-html="SectionData.placeBidModal.content"></p>
                    <div class="mb-3">
                        <label class="form-label">{{ SectionData.placeBidModal.labelText }}</label>
                        <input type="text" class="form-control form-control-s1" placeholder="Enter bid">
                    </div>
                    <div class="mb-3">
                        <label class="form-label" v-html="SectionData.placeBidModal.labelTextTwo"></label>
                        <input type="text" class="form-control form-control-s1" value="1">
                    </div>
                    <ul class="total-bid-list mb-4">
                        <li v-for="(list, i) in SectionData.placeBidModal.totalBidList" :key="i"><span>{{ list.title }}</span> <span>{{ list.price }}</span></li>
                    </ul>
                    <a :href="SectionData.placeBidModal.btnLink" class="btn btn-dark d-block">{{ SectionData.placeBidModal.btnText }}</a>
                </div><!-- end modal-body -->
            </div><!-- end modal-content -->
        </div><!-- end modal-dialog -->
    </div><!-- end modal-->
</div><!-- end page-wrap -->
</template>

<script>
import SectionData from '@/store/store.js'

import { trunc } from "@/utils/address";
import { safeIpfsToUrl } from "@/utils/nft";
import * as query from '@/contract/query'

export default {
  name: 'ProductDetail',
  data() {
        return{
            SectionData,
            id: this.$route.params.id,
            contract: this.$route.params.contract,
            nft: {},
            collection: {},
            owner: {},
            minter: {address:""}
         }
    },

    methods: {
        trunc(str, len) {
            return trunc(str, len)
        }
    },

    mounted() {
        query.queryRaw(this.contract, { all_nft_info: {token_id: this.id, include_expired: false} })
        .then(res => {
            console.log(res)
            this.nft = {...res.info, image: safeIpfsToUrl(res.info.image)}
            this.owner = {address: res.access.owner}
        });

        query.queryRaw(this.contract, { contract_info: {} })
        .then(res => {
            console.log(res)
            this.collection = res
        });

        query.queryRaw(this.contract, { minter: {} })
        .then(res => {
            console.log(res)
            this.minter = {address: res.minter}
        });
  }
}
</script>