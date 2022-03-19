<template>
    <div class="hero-wrap hero-wrap-5">
        <div class="container">
            <div class="row">
                <div class="col-md-9 mx-auto">
                    <div class="hero-content py-0 text-center">
                        <h1 class="hero-title hero-title-lg mb-4">{{ SectionData.heroDataSix.title }}</h1>
                        <p class="hero-text hero-text-s2 px-lg-5">{{ SectionData.heroDataSix.content }}</p>
                        <ButtonGroup :btns="SectionData.btnDataThree" classname="hero-btns"></ButtonGroup>
                        <div class="row g-gs justify-content-center counter-wrap">
                            <div class="col-sm-4" :class="item.className" v-for="item in SectionData.countItemData" :key="item.id">
                                <div class="count-item">
                                    <h3 class="count-title">{{ item.title }}</h3>
                                    <span class="count-text small">{{ item.text }}</span>
                                </div>
                            </div>
                        </div>
                    </div><!-- hero-content -->
                </div><!-- col -->
            </div>
        </div><!-- .container-->
    </div><!-- end hero-wrap -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
export default {
  name: 'HeroSeven',
  data () {
    return {
      SectionData
    }
  },
}
</script>