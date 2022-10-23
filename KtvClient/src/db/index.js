
const baseUrl = "http://192.168.68.54:8081"

export async function querySongs(request, pageNum, pageSize) {
	return await postData(`${baseUrl}/songs?page_num=${pageNum}&page_size=${pageSize}`, request)
} 

export async function queueSong(songId) {
	return await postData(`${baseUrl}/queue/${songId}`, {})
} 

export async function getSongQueue() {
	return await getData(`${baseUrl}/queue`)
} 

export async function prioritizeSong(songId) {
	return await putData(`${baseUrl}/queue/${songId}/prioritize`)
}

export async function deprioritizeSong(songId) {
	return await putData(`${baseUrl}/queue/${songId}/deprioritize`)
}

export async function deleteSongFromQ(songId) {
	return await deleteData(`${baseUrl}/queue/${songId}`)
}

export async function nextSong() {
	return await putData(`${baseUrl}/queue/next_song`)
}

async function postData(url = '', data = {}) {
	try {
		let response = await fetch(url, {
			method: 'POST',
			//mode: 'no-cors',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify(data),
		});
		return response.json()
	} catch (error) {
		console.log(error);
		return {}
	}
}

async function getData(url = '') {
	try {
		let response = await fetch(url, {
			method: 'GET', // or 'PUT'
			headers: {
				'Content-Type': 'application/json',
			}
		});
		return response.json()
	} catch (error) {
		console.log(error);
		return {}
	}
}

async function putData(url = '') {
	try {
		let response = await fetch(url, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
			}
		});
		return response.json()
	} catch (error) {
		console.log(error);
		return {}
	}
}


async function deleteData(url = '') {
	try {
		let response = await fetch(url, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
			}
		});
		return response.json()
	} catch (error) {
		console.log(error);
		return {}
	}
}