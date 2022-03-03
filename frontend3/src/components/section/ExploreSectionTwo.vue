<template>
    <section class="explore-section pt-4">
        <div class="container">
            <!-- filter -->
            <div class="filter-box pb-5">
                <div class="filter-box-filter justify-content-between align-items-center">
                    <div class="filter-box-filter-item">
                        <v-select class="generic-select generic-select-s3" v-model="category" :options="options" :clearable = false></v-select>
                     </div><!-- end filter-box-filter-item -->
                    <div class="filter-box-filter-item quicksearch-wrap">
                         <input type="text" v-model="name" placeholder="Search By Name" class="form-control form-control-s1"/>
                     </div><!-- end filter-box-filter-item -->
                    <div class="filter-box-filter-item ms-lg-auto filter-btn-wrap">
                         <div class="filter-btn-group">
                              <a href="#" class="btn filter-btn" :class="getActiveClass('recent')" @click.prevent="recent(); activeId = 'recent'">Recent</a>
                              <a href="#" class="btn filter-btn" :class="getActiveClass('low')" @click.prevent="lowToHigh(); activeId = 'low'">Price: Low</a>
                              <a href="#" class="btn filter-btn" :class="getActiveClass('high')" @click.prevent="highToLow(); activeId = 'high'">Price: High</a>
                          </div>
                     </div><!-- end filter-box-filter-item -->
                     <div class="filter-box-filter-item filter-mobile-action ms-lg-auto">
                          <div class="filter-box-search-mobile dropdown me-2">
                              <a class="icon-btn" href="#" data-bs-toggle="dropdown">
                                  <em class="ni ni-search"></em>
                              </a>
                              <div class="dropdown-menu dropdown-menu-end card-generic card-generic-s2 mt-2 p-3">
                                  <input type="text" v-model="name" placeholder="Search By Name" class="form-control form-control-s1"/>
                              </div>
                          </div><!-- end filter-box-search-mobile -->
                          <div class="filter-box-btn-group-mobile dropdown">
                              <a class="icon-btn" href="#" data-bs-toggle="dropdown">
                                  <em class="ni ni-filter"></em>
                              </a>
                              <div class="dropdown-menu dropdown-menu-end card-generic mt-2 p-3">
                                  <div class="filter-btn-group filter-btn-group-s1">
                                      <a href="#" class="btn filter-btn" :class="getActiveClass('recent')" @click.prevent="recent(); activeId = 'recent'">Recent</a>
                                      <a href="#" class="btn filter-btn" :class="getActiveClass('low')" @click.prevent="lowToHigh(); activeId = 'low'">Price: Low</a>
                                      <a href="#" class="btn filter-btn" :class="getActiveClass('high')" @click.prevent="highToLow(); activeId = 'high'">Price: High</a>
                                  </div>
                              </div>
                          </div><!-- end filter-box-btn-group-mobile -->
                      </div><!-- end filter-box-filter-item -->
                </div><!-- end filter-box-filter -->
            </div><!-- end filter-box -->
            <!-- Product -->
            <div class="row g-gs">
                <div class="col-xl-3 col-lg-4 col-sm-6" v-for="product in filterProducts" :key="product.id">
                     <Products :product="product"></Products>
                </div>
            </div><!-- end row -->
        </div><!-- .container -->
    </section><!-- end explore-section -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
import Products from '@/components/section/Products'
export default {
  name: 'ExploreSectionTwo',
  components: { 
    Products
  },
  data () {
    return {
      SectionData,
      category: 'all',
      name: '',
      activeId: 0,
      options: [
        'all',
        'art',
        'music',
        'games',
        'collectibles'
      ],
    }
  },
  methods: {
    // filter by category
    filterProductsByCategory(products){
      return products.filter(product => !product.category.indexOf(this.category))
    },
    // search item by keyword
    filterProductsByName(products) {
      return products.filter(product => !product.name.toLowerCase().indexOf(this.name.toLowerCase()))
    },
    // low to high
    lowToHigh(){
        this.SectionData.productData.products.sort((a, b)=>(a.price > b.price) ? 1 : -1)
    },
    // high to low
    highToLow(){
        this.SectionData.productData.products.sort((a, b)=>(a.price < b.price) ? 1 : -1)
    },
    // recent item
    recent(){
        this.SectionData.productData.products.sort((a, b)=>(a.id < b.id) ? 1 : -1)
    },
    // add active class to button
    getActiveClass(id) {
      if (id == this.activeId) {
        return "active";
      }else {
        return "";
      }
    },
  },
  computed: {
    filterProducts(){
      var selectedCategory = this.category;
      if(selectedCategory === 'all') {
        return this.SectionData.productData.products,
               this.filterProductsByName(this.SectionData.productData.products);
      }else{
        return this.filterProductsByName(this.filterProductsByCategory(this.SectionData.productData.products))
      }
    },
  },
  created(){
    this.getActiveClass(this.activeId = 'recent');
    this.recent();
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