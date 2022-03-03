<template>
    <div class="row g-gs">
        <div class="col-lg-6" v-for="item in SectionData.sellBuyData" :key="item.id">
            <div class="card card-sell-buy card-full">
                <div class="card-image">
                    <img :src="item.img" alt="">
                </div>
                <div class="card-body px-0">
                    <h4 class="mb-2">{{ item.title }}</h4>
                    <p class="mb-4">{{ item.content }}</p>
                    <router-link :to="item.btnLink" class="btn btn-sm btn-dark">{{ item.btnText }}</router-link>
                </div>
            </div><!-- end card -->
        </div><!-- end col-lg-4 -->
    </div><!-- row -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'SellBuy',
  data () {
    return {
      SectionData
    }
  }
}
</script>