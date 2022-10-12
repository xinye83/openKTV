import Vue from 'vue'
import Router from 'vue-router'
import Songs from '@/components/Songs'
import ProductPage from '@/components/ProductPage'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Songs',
      component: Songs
    },
    {
      path: '/product/:id',
      name: 'ProductPage',
      component: ProductPage
    }
  ]
})
