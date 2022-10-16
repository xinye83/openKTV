<template>
  <div id="app">
    <Navbar/>
    <div class="container">
      <router-view/>
    </div>
  </div>
</template>

<script>
import Navbar from '@/components/Navbar'
import Playlist from '@/components/Playlist'
import Songs from '@/components/Songs'
export default {
  name: 'App',
  components:{
    Navbar,
    Playlist,
    Songs
  }
}
</script>

<style>
</style>
