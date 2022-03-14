<template>
    <div class="row g-gs">
        <div class="col-lg-3 col-sm-6" v-for="item in SectionData.creatorData.creatorListTwo" :key="item.id">
            <div class="card-creator-v card-creator-v-wbg card-creator-v-wbg-s1 pt-0 pb-4" :class="classname">
                <div class="card-creator-info">
                    <router-link :to="item.path" class="avatar avatar-1 flex-shrink-0 mb-2">
                        <img :src="item.img" alt="avatar">
                        <span class="card-creator-number">{{ item.id }}</span>
                    </router-link>
                    <div class="flex-grow-1">
                        <router-link :to="item.path" class="card-title">{{ item.title }}</router-link>
                        <span class="d-block fw-medium card-amount">{{ item.subTitle }}</span>
                    </div>
                </div>
            </div><!-- end card -->
        </div><!-- end col-lg-4 -->
    </div><!-- row -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'CreatorsFour',
  props: ['classname'],
  data () {
    return {
      SectionData
    }
  }
}
</script>
