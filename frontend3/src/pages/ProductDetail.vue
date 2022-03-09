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
                                <img :src="wrapper.info.image" alt="" class="w-100 rounded-3">
                            </div><!-- end item-detail-img-container -->

                        </div><!-- end item-detail-content -->
                    </div><!-- end col -->

                    <!-- DETAILS-->
                    <div class="col-lg-6">
                        <div class="item-detail-content mt-4 mt-lg-0">
                            <h1 class="item-detail-title mb-2">{{ wrapper.info.name }}</h1>
                            <div v-if="wrapper.inSale" class="item-detail-meta d-flex flex-wrap align-items-center mb-3">
                                <span class="item-detail-text-meta">FOR SALE</span>
                                <span class="dot-separeted"></span>
                                <span class="item-detail-text-meta">{{ wrapper.offer.price.amount }} {{ wrapper.offer.price.denom }}</span>
                            </div>
                            <p class="item-detail-text mb-4">{{ wrapper.info.description }}</p>
                            <div class="item-credits">
                                <div class="row g-4">
                                    <div class="col-xl-6">
                                        <div class="card-media card-media-s1">
                                            <router-link v-if="wrapper.minter.avatar" :to="'/profile/'+wrapper.minter.address" class="card-media-img flex-shrink-0 d-block">
                                                <img :src="wrapper.minter.avatar" alt="avatar">
                                            </router-link>
                                            <div v-if="wrapper.minter.address" class="card-media-body">
                                                <router-link :to="'/profile/'+wrapper.minter.address" class="fw-semibold">{{ trunc(wrapper.minter.address, 18) }}</router-link>
                                                <p class="fw-medium small">Minter</p>
                                            </div>
                                        </div><!-- end card -->
                                    </div><!-- end col-->
                                    <div class="col-xl-6">
                                        <div class="card-media card-media-s1">
                                            <router-link v-if="wrapper.collection.avatar" :to="'/collections/'+contract" class="card-media-img flex-shrink-0 d-block">
                                                <img :src="wrapper.collection.avatar" alt="avatar">
                                            </router-link>
                                            <div class="card-media-body">
                                                <router-link :to="'/collections/'+contract" class="fw-semibold">{{ wrapper.collection.name }}</router-link>
                                                <p class="fw-medium small">Collection</p>
                                            </div>
                                        </div><!-- end card -->
                                    </div><!-- end col-->
                                    <div class="col-xl-12">
                                        <div class="card-media card-media-s1">
                                            <router-link v-if="wrapper.owner.avatar" :to="'/profile/'+wrapper.owner.address" class="card-media-img flex-shrink-0 d-block">
                                                <img :src="item.avatar" alt="avatar">
                                            </router-link>
                                            <div class="card-media-body">
                                                <router-link :to="'/profile/'+wrapper.owner.address" class="fw-semibold">{{ wrapper.owner.address }}</router-link>
                                                <p class="fw-medium small">Owner</p>
                                            </div>
                                        </div><!-- end card -->
                                    </div><!-- end col-->
                                </div><!-- end row -->
                            </div><!-- end row -->
                            <div class="item-detail-btns mt-4">
                                <ul class="btns-group d-flex">
                                    <li v-if="wrapper.inSale" class="flex-grow-1">
                                        <a v-if="IsConnected()" href="#" @click.prevent="() => showModal()" class="btn btn-dark d-block">Buy</a>
                                        <!-- <button v-if="IsConnected()" class="btn btn-dark d-block w-200">Buy</button> -->
                                        <span v-else class="text-orange">Connect your wallet to buy or place a bid</span>
                                    </li>
                                    <!-- <li class="flex-grow-1">
                                        <div class="dropdown">
                                            <a href="#" class="btn bg-dark-dim d-block" data-bs-toggle="dropdown">{{ SectionData.itemDetailData.btnTextTwo }}</a>
                                            <div class="dropdown-menu card-generic p-2 keep-open w-100 mt-1">
                                                <router-link :to="icon.path" class="dropdown-item card-generic-item" v-for="(icon, i) in SectionData.socialShareList" :key="i"><em class="ni me-2" :class="icon.btnClass"></em>{{ icon.title }}</router-link>
                                            </div>
                                        </div>
                                    </li> -->
                                </ul>
                            </div><!-- end item-detail-btns -->
                        </div><!-- end item-detail-content -->
                    </div><!-- end col -->
                </div><!-- end row -->
            </div><!-- .container -->
            <!-- Modal -->
            <div v-if="wrapper.inSale" class="modal fade" id="placeBidModal" tabindex="-1" aria-hidden="true">
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h4 class="modal-title">Buy Nft</h4>
                            <button type="button" class="btn-close icon-btn" data-bs-dismiss="modal" aria-label="Close">
                                <em class="ni ni-cross"></em>
                            </button>
                        </div><!-- end modal-header -->
                        <div class="modal-body">
                            <p class="mb-3">
                                You are about to buy <strong>{{ wrapper.collection.name }} - {{ wrapper.info.name }}</strong> 
                                from <strong> {{ wrapper.owner.address }}</strong>
                            </p>
                            <ul class="total-bid-list mb-4">
                                <li><span>Price</span> <span>{{ wrapper.offer.price.amount }} {{ wrapper.offer.price.denom }}</span></li>
                            </ul>
                            <p class="mb-3 text-red .smaller lh">
                                The purchase is not refundable and you will wait some time before the transaction is processed by <strong class="text-red">Terra</strong>
                            </p>
                            <button v-on:click="() => buy()" class="btn btn-primary d-block">Buy</button>
                        </div><!-- end modal-body -->
                    </div><!-- end modal-content -->
                </div><!-- end modal-dialog -->
            </div><!-- end modal-->
    </section><!-- end item-detail-section -->
    <!-- Footer  -->
    <Footer classname="bg-dark on-dark"></Footer>
</div><!-- end page-wrap -->
</template>

<script>
import SectionData from '@/store/store.js'
import { Modal } from 'bootstrap';
import { Fee } from "@terra-money/terra.js";

import {exec} from '@/contract/execute';

import { trunc } from "@/utils/address";
import { NftWrapper } from '@/models/nft-wrapper';
import walletController from "@/mixins/walletController.js"

export default {
  name: 'ProductDetail',
  mixins: [walletController],

  data() {
        return{
            SectionData,
            id: this.$route.params.id,
            contract: this.$route.params.contract,
            wrapper: new NftWrapper(this.$route.params.contract, this.$route.params.id),
            modal: undefined
         }
    },

    methods: {
        showModal() {
            if (!this.modal) {
                this.modal = new Modal(document.getElementById('placeBidModal'));
            }
            this.modal.show()
        },

        trunc(str, len) {
            return trunc(str, len)
        },

        buy() {
            const wallet = this.GetWallet();
            if (!wallet) {
                return;
            }
            
            console.log(wallet)

            const coins = {};
            coins[this.wrapper.offer.priceCanonical.denom] = this.wrapper.offer.priceCanonical.amount;
            const fees = {};
            if (this.wrapper.offer.priceCanonical.denom != "uluna") {
                fees[this.wrapper.offer.priceCanonical.denom] = Math.trunc(parseInt(this.wrapper.offer.priceCanonical.amount)/1000);
            }

            exec(wallet, "marketplace", 
                { buy: 
                    {
                        offering_id: this.wrapper.offer.id, 
                        payment: {
                            denom: this.wrapper.offer.priceCanonical.denom, 
                            amount: this.wrapper.offer.priceCanonical.amount
                        } 
                    }
                },
                coins,
                new Fee(200000, { uluna: 10000,  ...fees})
                )
                .then(res => {
                    console.log(res)
                    this.$notify({
                        title: 'Success!!',
                        text: 'You successfully purchased ' + this.wrapper.info.name,
                        type: 'success',
                    });
                    this.wrapper.load(true);
                })
                .catch(err => {
                    console.log(err);
                    this.$notify({
                        title: 'Error occurred',
                        text: 'Try to refresh the page and purchase again',
                        type: 'error',
                    });
                })
                .finally(() => {
                    this.modal.hide();
                })
                
            this.$notify({
                title: 'Processing transaction',
                text: 'Wait some time on this page',
                duration: 20000
            });
        }
    },

    mounted() {
        this.wrapper.load();
    }
}
</script>