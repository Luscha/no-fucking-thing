<template>
    <div class="hero-wrap hero-wrap-4">
        <div class="container">
            <div class="row align-items-center flex-lg-row-reverse justify-lg-content-center">
                <div class="col-lg-6 col-sm-9">
                    <div class="row gx-4">
                        <div class="col-xl-8">
                            <div class="card card-s2" v-for="product in showItemSingle" :key="product.id">
                                <div class="card-image">
                                    <img :src="product.imgLg" alt="" class="card-img-top card-img-height">
                                </div>
                                <div class="card-body">
                                    <h5 class="card-title text-truncate">{{ product.title }}</h5>
                                    <div class="card-author d-flex align-items-center">
                                        <span class="me-1 card-author-by">By</span>
                                        <router-link :to="product.authorLink" class="author-link">{{ product.author }}</router-link>
                                    </div><!-- end card-author -->
                                </div><!-- end card-body -->
                                <router-link
                                class="details"
                                :to="{
                                    name: 'ProductDetail',
                                    params: {
                                        id: product.id,
                                        title: product.title,
                                        metaText: product.metaText,
                                        price: product.price,
                                        priceTwo: product.priceTwo,
                                        imgLg: product.imgLg,
                                        metaText: product.metaText,
                                        metaTextTwo: product.metaTextTwo,
                                        metaTextThree: product.metaTextThree,
                                        content: product.content,
                                    }
                                }"
                            >
                            </router-link>
                            </div><!-- end card -->
                        </div>
                        <div class="col-xl-4 d-xl-block d-none">
                            <div class="card card-s2" v-for="product in showItem" :key="product.id">
                                <div class="card-image">
                                    <img :src="product.imgLg" alt="" class="card-img-top">
                                </div>
                                <div class="card-body">
                                    <h5 class="card-title text-truncate">{{ product.title }}</h5>
                                    <div class="card-author d-flex align-items-center">
                                            <span class="me-1 card-author-by">By</span>
                                        <router-link :to="product.authorLink" class="author-link">{{ product.author }}</router-link>
                                    </div><!-- end card-author -->
                                </div><!-- end card-body -->
                                <router-link
                                class="details"
                                :to="{
                                    name: 'ProductDetail',
                                    params: {
                                    id: product.id,
                                    title: product.title,
                                    metaText: product.metaText,
                                    price: product.price,
                                    priceTwo: product.priceTwo,
                                    imgLg: product.imgLg,
                                    metaText: product.metaText,
                                    metaTextTwo: product.metaTextTwo,
                                    metaTextThree: product.metaTextThree,
                                    content: product.content,
                                    }
                                }"
                            >
                            </router-link>
                            </div><!-- end card -->
                        </div><!-- end col -->
                    </div><!-- end row -->
                </div><!-- end col -->
                <div class="col-lg-6 col-sm-9">
                    <div class="hero-content pt-lg-0 pb-0 mt-lg-n4">
                        <h6 class="hero-meta text-uppercase text-primary mb-3">{{ SectionData.heroDataFour.subTitle }}</h6>
                        <h1 class="hero-title">{{ SectionData.heroDataFour.title }}</h1>
                        <ul class="list-item list-item-icon list-item-hero">
                            <li><span class="ni ni-check icon icon-circle icon-wbg icon-xs"></span> Create, Buy, Sell and Earn with NFTs</li>
                            <li><span class="ni ni-check icon icon-circle icon-wbg icon-xs"></span> Faster and cheaper fees under $1</li>
                            <li><span class="ni ni-check icon icon-circle icon-wbg icon-xs"></span> Stake your earnings and earn more.</li>
                        </ul>
                        <ButtonGroup :btns="SectionData.btnDataThree" classname="hero-btns"></ButtonGroup>
                    </div><!-- hero-content -->
                </div><!-- col-lg-6 -->
            </div>
        </div><!-- .container-->
    </div><!-- end hero-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'HeroFive',
  data () {
    return {
      SectionData
    }
  },
  computed: {
    showItem() {
        return this.SectionData.productData.products.filter(item => item.itemToShowTwo === 'show')
    },
    showItemSingle() {
        return this.SectionData.productData.products.filter(item => item.itemToShowSingle === 'show')
    },
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
 .author-link {
   z-index: 2;
   position: relative;
 }
</style>