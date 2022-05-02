<template>
  <section class="section-space how-it-work-section">
      <div class="container">
          <!-- section heading -->
          <div class="section-head" :class="`text-center`">
              <h3 :class="`mb-3`" v-html="howItWorkData.title"></h3>
          </div><!-- end section-head -->
          <div class="row g-gs justify-content-center">
              <div class="col-10 col-sm-6 col-md-6" v-for="item in howItWorkData.howItWorkList" :key="item.id" :class="classname">
                  <div class="card-htw text-center" :class="classnameTwo">
                      <span :class="item.icon"></span>
                      <h4 class="mb-3 card-text" v-html="item.title"></h4>
                      <p class="card-text-s1" v-html="item.content"></p>
                  </div>
              </div><!-- end col -->
          </div><!-- end row -->
      </div><!-- end container -->
  </section><!-- end how-it-work-section -->
</template>

<script>
export default {
  name: 'HowItWork',
  props: ['classname', 'classnameTwo', 'gutterBottom'],
  data () {
    return {
      // how It Work Data
      howItWorkData: {
        title: "Mint, buy and sell Nfts or discover their unknown protential through a new way of investing",
        howItWorkList: [
          {
            id: 1,
            icon: 'icon ni ni-wallet icon-lg icon-circle shadow-sm icon-wbg mx-auto mb-4 text-primary',
            title: '<a href="https://docs.terra.money/docs/learn/terra-station/download/README.html" target="_blank" class="btn-link">Set up your wallet</a>',
          },
          {
            id: 2,
            icon: 'icon ni ni-file-text icon-lg icon-circle shadow-sm icon-wbg mx-auto mb-4 text-danger',
            title: 'Create NFTs collection',
          },
          {
            id: 3,
            icon: 'icon ni ni-money icon-lg icon-circle shadow-sm icon-wbg mx-auto mb-4 text-info',
            title: 'List them and more',
          }
        ],
      }
    }
  }
}
</script>
