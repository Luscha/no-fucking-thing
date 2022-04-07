<template>
  <section class="section-space how-it-work-section">
      <div class="container">
          <!-- section heading -->
          <SectionHeading classname="text-center" :text="howItWorkData.title" :content="howItWorkData.subtitle" :isMargin="gutterBottom"></SectionHeading>
          <div class="row g-gs justify-content-center">
              <div class="col-10 col-sm-6 col-md-6" v-for="item in howItWorkData.howItWorkList" :key="item.id" :class="classname">
                  <div class="card-htw text-center" :class="classnameTwo">
                      <span :class="item.icon"></span>
                      <h4 class="mb-3">{{ item.title }}</h4>
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
        title: 'Create and sell your NFTs',
        content: 'This is just a simple text made for this unique and awesome template, you can replace it with any text. It is a long established fact.',
        titleTwo: 'Mint, Buy and Sell or Just Launch <br> Your Own NFT Store',
        titleThree: 'Get Started in 3 simple steps',
        btnText: 'Create Store',
        howItWorkList: [
          {
            id: 1,
            icon: 'icon ni ni-wallet icon-lg icon-circle shadow-sm icon-wbg mx-auto mb-4 text-primary',
            title: 'Set up your wallet',
            content: 'Once you’ve set up your wallet of choice, connect it to EnftyMart by clicking the'
          },
          {
            id: 2,
            icon: 'icon ni ni-file-text icon-lg icon-circle shadow-sm icon-wbg mx-auto mb-4 text-danger',
            title: 'Create collection',
            content: 'Click <a href="profile" class="btn-link">My Collections</a> and set up your collection. Add social links, a description'
          },
          {
            id: 3,
            icon: 'icon ni ni-camera icon-lg icon-circle shadow-sm icon-wbg mx-auto mb-4 text-info',
            title: 'Add your NFTs',
            content: 'Upload your work (image, video, audio, or 3D art), add a title and description'
          }
        ],
      }
    }
  }
}
</script>
