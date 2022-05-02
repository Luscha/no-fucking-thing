import { createWebHistory, createRouter } from "vue-router";

const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import('../pages/Home-v2.vue'),
  },
  {
    path: "/mint-gen0",
    name: "Mint Gen0",
    component: () => import('../pages/MintGen0.vue'),
  },
  {
    path:'/product-details-v1-:contract-:id',
    name:'ProductDetail',
    component: () => import('../pages/ProductDetail.vue'),
    props: true
  },
  {
    path: '/explore',
    name: 'explore',
    component: () => import('../pages/Explore.vue')
  },
  {
    path: '/wallet',
    name: 'wallet',
    component: () => import('../pages/Wallet.vue')
  },
  {
    path: '/profile/:address',
    name: 'profile',
    component: () => import('../pages/Profile.vue')
  },
  {
    path: '/me/:section',
    name: 'me',
    component: () => import('../pages/ProfileMe.vue')
  },
  {
    path: '/faq',
    name: 'faq',
    component: () => import('../pages/Faq.vue')
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior (to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    } else {
      return {
        left: 0,
        top: 0
      }
    }
  }
});

export default router;