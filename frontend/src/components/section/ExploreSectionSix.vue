<template>
    <section class="explore-section pt-4">
        <div class="container">
            <!-- filter -->
            <div class="filter-box d-flex flex-wrap align-items-center justify-content-between pb-5 pb-sm-4">
                <h2 class="me-3 my-3">Filter by</h2>
                <div class="filter-btn-group">
                    <a href="#" class="btn btn-sm filter-btn" :class="tab.class" v-for="tab in filterMenu" @click.prevent="setTab(tab, tab.id)"  :key="tab.id">{{ tab.title }}</a>
                </div>
            </div><!-- end filter-box -->
            <div class="row g-gs">
              <div class="col-xl-3 col-lg-4 col-sm-6" v-for="product in filteredData" :key="product.id">
                   <ProductsSix :product="product"></ProductsSix>
              </div><!-- end col -->
            </div><!-- end row -->
        </div><!-- .container -->
    </section><!-- end explore-section -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import ProductsSix from '@/components/section/ProductsSix'

export default {
  name: 'ExploreSectionSix',
  components: { 
    ProductsSix
  },
  data () {
    return {
      SectionData,
      filterMenu: [
        {
            id: 1,
            title: 'All',
            class: 'active',
            options: [
                {
                    category: 'art'
                },
                {
                    category: 'music'
                },
                {
                    category: 'collectibles'
                },
                {
                    category: 'games'
                },
            ]
        },
        {
            id: 2,
            title: 'Artwork',
            class: '',
            options: [
                {
                    category: 'art'
                }
            ]
        },
        {
            id: 3,
            title: 'Music',
            class: '',
            options: [
                {
                    category: 'music'
                }
            ]
        },
        {
            id: 4,
            title: 'Collectibles',
            class: '',
            options: [
                {
                    category: 'collectibles'
                }
            ]
        },
      ],
      selectedTab: null,
      previous_active_id: 1
    }
  },
  methods: {
    setTab(tab, id) {
      this.selectedTab = tab;
      if (this.previous_active_id === id) return;
      this.filterMenu.find(menu => menu.id === this.previous_active_id).class = '';
      this.filterMenu.find(menu => menu.id === id).class = 'active';
      this.previous_active_id = id;
    },
  },
  computed: {
      filteredData () {
        return this.SectionData.productData.products.filter(data => {
        if (this.selectedTab === null) return true
        const opts = this.selectedTab.options.map(opt => opt.category)
        return opts.includes(data.category)
      })
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
