use serde::{Deserialize, Serialize};


#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Song {
    pub id: u64,
    pub name: String,
    #[sqlx(flatten)]
    pub artist: Artist,
    //pub state: State,
    pub url: String,
    //pub played_count: usize,

}

// #[derive(Serialize, Deserialize, Display)]
// pub enum State {
//     New,
//     Error,
// }


#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Artist {
    pub id: u64,
    #[sqlx(rename = "artist_name")]
    pub name: String,
    pub region: String,
}

impl Song {
    // pub fn new_youtube(name: String, artist: String, url: String) -> Song {
    //     Song {
    //         uuid: Uuid::new_v4().to_string(),
    //         name,
    //         artist,
    //         state: State::New,
    //         url,
    //         played_count: 0,
    //     }
    // }
}