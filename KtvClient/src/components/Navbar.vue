<template>
	<div class="header">
		<nav class="navbar navbar-light bg-light navbar-expand-lg">
			<div class="container">
				<router-link class="navbar-brand" to="/">Open KTV</router-link>
				<button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav"
					aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
					<span class="navbar-toggler-icon"></span>
				</button>
				<div class="collapse navbar-collapse" id="navbarNav">

					<ul class="navbar-nav">
						<li class="nav-item active">
							<!-- <a class="nav-link" href="#">Playlist<span class="sr-only">(current)</span></a> -->
							<b-nav-item to="/" exact exact-active-class="active">Playlist</b-nav-item>
						</li>
						<li class="nav-item">
							<!-- <a class="nav-link" href="#">Songs</a> -->
							<b-nav-item to="/songs" exact exact-active-class="active">Songs</b-nav-item>
						</li>
						<li class="nav-item">
							<!-- <a class="nav-link" href="#">Artists</a> -->
							<b-nav-item to="/artists" exact exact-active-class="active">Artists</b-nav-item>
						</li>
					</ul>

				</div>
				<b-icon icon="skip-end-btn-fill" @click="onNextSong()" variant="secondary" font-scale="4"></b-icon>
			</div>
		</nav>
	</div>
</template>

<script>
import { nextSong } from '../db'
import { store } from '../db/store'

export default {
	name: 'OpenKTV',
	data() {
		return {
			store,
		}
	},
	methods: {
		setData(songs) {
			this.store.currentPlaylist = songs.map(x => {
				return { ...x, artist: x.artist_name, name: x.song_name, isPrioritized: x.prioritized_at != null }
			})
		},
		async onNextSong() {
			nextSong()
			.then(s => {
				this.setData(s)
			})
			.catch(_ => {
				console.log("unable to play next song..")
				this.onNextSong()
			})
		}
	}
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1,
h2 {
	font-weight: normal;
}

ul {
	list-style-type: none;
	padding: 0;
}

li {
	display: inline-block;
	margin: 0 10px;
}

a {
	color: #42b983;
}
</style>
