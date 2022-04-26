<template>
    <div class="container">
                <div class="section-space-sm">
                    <div class="row">
                        <div class="col-lg-8 col-md-9 me-auto">
                            <div class="footer-item mb-5 mb-lg-0">
                                <!-- logo link -->
                                <LogoLink class="footer-logo-link"></LogoLink>
                                <p class="my-4 footer-para" v-if="SectionData.footerData.content">{{ SectionData.footerData.content }}</p>
                            </div><!-- end footer-item -->
                        </div><!-- end col-lg-3 -->
                        <div class="col-lg-3">
                            <ul class="styled-icon">
                                <li><a href="#"><em class="icon ni ni-twitter"></em></a></li>
                                <li><a href="#"><em class="icon ni ni-facebook-f"></em></a></li>
                                <li><a href="#"><em class="icon ni ni-instagram"></em></a></li>
                                <li><a href="#"><em class="icon ni ni-pinterest"></em></a></li>
                            </ul>
                        </div>
                    </div><!-- end row -->
                </div>
            </div><!-- .container -->
</template>
<script>
// Import component data. You can change the data in the store to reflect in all component
import SectionData from '@/store/store.js'

export default {
  name: 'FooterSection',
  data () {
    return {
      SectionData
    }
  }
}
</script>
