
const baseUrl = "http://192.168.68.54:8081"

export async function querySongs(request, pageNum, pageSize) {
	return await postData(`${baseUrl}/songs?page_num=${pageNum}&page_size=${pageSize}`, request)
} 

export async function queueSong(songId) {
	return await postData(`${baseUrl}/queue/${songId}`, {})
} 

async function postData(url = '', data = {}) {
	try {
		let response = await fetch(url, {
			method: 'POST', // or 'PUT'
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