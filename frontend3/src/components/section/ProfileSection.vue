<template>
<section class="author-section section-space">
      <div class="container">
          <div class="row">
              <div class="col-xl-3">
                  <AuthorSidebar
                  :content="SectionData.authorSidebarDataTwo.sidebarWidget.content"
                  :followingnum="SectionData.authorSidebarDataTwo.sidebarWidget.followingNum"
                  :followingtext="SectionData.authorSidebarDataTwo.sidebarWidget.followingText"
                  :followernum="SectionData.authorSidebarDataTwo.sidebarWidget.followerNum"
                  :followertext="SectionData.authorSidebarDataTwo.sidebarWidget.followerText"
                  :avatars="SectionData.authorSidebarDataTwo.sidebarWidget.followersAvatar"
                  :links="SectionData.authorSidebarDataTwo.sidebarWidgetTwo.links"
                  :datetext="SectionData.authorSidebarDataTwo.sidebarWidgetThree.date"
                  ></AuthorSidebar>
              </div><!-- end col -->
              <div class="col-xl-9 ps-xl-4">
                  <div class="author-items-wrap">
                      <h3>{{ SectionData.profileData.title }}</h3>
                      <div class="gap-2x"></div><!-- end gap -->
                      <div class="row g-gs">
                          <div class="col-md-4" v-for="product in SectionData.productData.products" :key="product.id">
                          <div class="card card-full">
                              <div class="card-image">
                                  <img :src="product.img" class="card-img-top" alt="art image">
                              </div>
                              <div class="card-body p-4">
                                  <h5 class="card-title text-truncate mb-0">{{ product.title  }}</h5>
                                  <div class="card-author mb-1 d-flex align-items-center">
                                      <span class="me-1 card-author-by">By</span>
                                      <div class="custom-tooltip-wrap">
                                          <router-link :to="product.authorLink" class="custom-tooltip author-link">{{ product.author }}</router-link>
                                      </div><!-- end custom-tooltip-wrap -->
                                  </div><!-- end card-author -->
                                  <div class="card-price-wrap d-flex align-items-center justify-content-between mb-3">
                                      <div class="me-2">
                                          <span class="card-price-title">Price</span>
                                          <span class="card-price-number">&dollar;{{ product.price }}</span>
                                      </div>
                                      <div class="text-sm-end">
                                          <span class="card-price-title">Current bid</span>
                                          <span class="card-price-number">{{ product.priceTwo }} ETH</span>
                                      </div>
                                  </div><!-- end card-price-wrap -->
                                  <a href="#" class="btn btn-sm bg-danger-dim" data-bs-toggle="modal" data-bs-target="#deleteModal">Delete</a>
                              </div><!-- end card-body -->
                              <router-link
                                  class="details"
                                    :to="{
                                      name: 'ProductDetail',
                                      params: {
                                          id: product.id,
                                          title: product.title,
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
                      </div><!-- row -->
                  </div><!-- end author-items-wrap -->
              </div><!-- end col-lg-8 -->
          </div><!-- end row -->
      </div><!-- .container -->
      <!-- Modal -->
      <div class="modal fade" id="deleteModal" tabindex="-1" aria-hidden="true">
          <div class="modal-dialog modal-dialog-centered">
              <div class="modal-content">
                  <div class="modal-body text-center">
                      <img :src="SectionData.deleteModal.img" alt="" class="mb-3">
                      <h4 class="modal-tilte mb-2">{{ SectionData.deleteModal.title }}</h4>
                      <p class="modal-text">{{ SectionData.deleteModal.content}}</p>
                  </div><!-- end modal-body -->
                  <div class="modal-footer">
                      <button type="button" class="btn btn-sm" data-bs-dismiss="modal">{{ SectionData.deleteModal.btnText }}</button>
                      <button type="button" class="btn btn-sm btn-danger" data-bs-dismiss="modal">{{ SectionData.deleteModal.btnTextTwo }}</button>
                  </div>
              </div><!-- end modal-content -->
          </div><!-- end modal-dialog -->
      </div><!-- end modal-->
      <!-- Modal -->
  </section><!-- end author-section -->
</template>

<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'
export default {
  name: 'ProfileSection',
  data () {
    return {
      SectionData,
    }
  },
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
 .btn,
 .author-link,
 .card-price-wrap {
   z-index: 2;
   position: relative;
 }
</style>