import Vue from 'vue'
import Router from 'vue-router'
import Playlist from '@/components/Playlist'
import Songs from '@/components/Songs'
import ProductPage from '@/components/ProductPage'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Playlist',
      component: Playlist
    },
    {
      path: '/songs',
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

