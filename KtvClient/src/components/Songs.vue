<template>
  <div class="products">
    <h3>Songs</h3>

    <div class="search-bar mt-3">

    <b-input-group>
      <b-form-input v-model="searchStr" placeholder="搜索歌名..."></b-form-input>
      <b-input-group-append>
        <b-button variant="primary" v-on:click="getItems">Search</b-button>
      </b-input-group-append>
      </b-input-group>

    </div>
    <div class="card mt-5">
      <template>
        <div class="overflow-auto">
          <!-- <p class="mt-3 text-center">Current Page: {{ currentPage }}</p> -->

          <b-table id="my-table" :busy.sync="isBusy" :items="items" :per-page="perPage" :current-page="currentPage"
            :fields="fields" small>
            <template v-slot:cell(actions)="{ item }">
              <span>
                <template v-if="item.is_queued">
                  <b-icon icon="plus-square-fill" variant="secondary" font-scale="1.8"></b-icon>
                </template>
                <template v-else>
                  <b-icon icon="plus-square-fill" @click="onOrderSong(item)" style="color: #7952b3;" font-scale="1.8"></b-icon>
                </template>
                
              </span>
            </template>
          </b-table>

          <div class="mt-3">
            <!-- <h6 class="text-center">Fill alignment</h6> -->
            <!-- <b-pagination v-model="currentPage" :total-rows="rows" align="fill"></b-pagination> -->
            <b-pagination v-model="currentPage" :total-rows="rows" :per-page="perPage" align="fill"
              aria-controls="my-table">
            </b-pagination>

          </div>
        </div>
      </template>

    </div>

  </div>
</template>

<script>
import { querySongs, queueSong } from '../db'
export default {
  name: 'Songs',
  data() {
    return {
      searchStr: '',

      fields: ['name', 'artist', "actions"],
      items: [],
      perPage: 5,
      currentPage: 1,
      isBusy: false,
    }
  },
  created() {
    this.getItems()
  },
  computed: {
    sortedItems() {
      return this.items.slice().sort((a, b) => {
        return a.id - b.id
      })
    },

    rows() {
      return this.items.length
    }
  },
  methods: {
    async getItems() {
      this.isBusy = true
      let songs = await querySongs({
        name: this.searchStr,
        artist: ""
      }, 0, 1000)
      this.isBusy = false
      this.items = songs.items.map(x => {
        return { ...x, artist: x.artist.name, totalCount: x.total_count }
      })
      console.log("querySongs response:", songs)
    },
    onSubmit() {


    },

    async onOrderSong(item) {
      console.log("ordering song:", item)
      await queueSong(item.id)
      // todo check if successful
      this.items = this.items.map(it => it.id == item.id ? { ...it, is_queued: true} : it)
    },

    onDelete(id) {

    },
    onEdit(product) {
      this.editId = product.id
      this.editProductData.product_id = product.product_id
      this.editProductData.product_name = product.product_name
      this.editProductData.product_price = product.product_price
    },
    onCancel() {
      this.editId = ''
      this.editProductData.product_id = ''
      this.editProductData.product_name = ''
      this.editProductData.product_price = ''
    },
    onEditSubmit(id) {
      db.collection("products").doc(id).set(this.editProductData).then(
        this.getProducts)
      this.editId = ''
      this.editProductData.product_id = ''
      this.editProductData.product_name = ''
      this.editProductData.product_price = ''
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  text-align: center;
  margin-top: 30px;
  margin-bottom: 20px;
}

.icon {
  margin-right: 10px;
}

.icon i {
  cursor: pointer;
}
</style>
