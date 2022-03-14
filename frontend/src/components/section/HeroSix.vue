<template>
    <div class="hero-wrap hero-wrap-5">
        <div class="container">
            <div class="row flex-md-row-reverse align-items-center">
                <div class="col-lg-5 ms-lg-auto" v-for="product in showItemSingle" :key="product.id">
                    <ProductsThree :product="product"></ProductsThree>
                </div><!-- end col -->
                <div class="col-lg-6">
                    <div class="hero-content pt-lg-0 pb-0">
                        <h1 class="hero-title mb-4">{{ SectionData.heroDataFive.title }}</h1>
                        <p class="hero-text mb-4 pb-1">{{ SectionData.heroDataFive.content }}</p>
                        <ButtonGroup :btns="SectionData.btnDataThree" classname="hero-btns"></ButtonGroup>
                        <div class="gap-2x"></div>
                        <hr class="my-0">
                        <div class="gap-2x"></div>
                        <div class="row g-gs">
                            <div class="col-sm-4" :class="item.className" v-for="item in SectionData.countItemData" :key="item.id">
                                <div class="count-item">
                                    <h3 class="count-title">{{ item.title }}</h3>
                                    <span class="count-text small">{{ item.text }}</span>
                                </div>
                            </div>
                        </div>
                    </div><!-- hero-content -->
                </div><!-- col-lg-6 -->
            </div>
        </div><!-- .container-->
    </div><!-- end hero-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import ProductsThree from '@/components/section/ProductsThree'

export default {
  name: 'HeroSix',
  components: { 
    ProductsThree 
  },
  data () {
    return {
      SectionData
    }
  },
  computed: {
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