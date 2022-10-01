use serde::{Deserialize, Serialize};
use uuid::Uuid;
use strum_macros::{Display};


#[derive(Serialize, Deserialize)]
pub struct Song {
    pub uuid: String,
    pub name: String,
    pub artist: String,
    pub state: State,
    pub url: String,
    pub played_count: usize,

}

#[derive(Serialize, Deserialize, Display)]
pub enum State {
    New,
    Error,
}


impl Song {
    pub fn new_youtube(name: String, artist: String, url: String) -> Song {
        Song {
            uuid: Uuid::new_v4().to_string(),
            name,
            artist,
            state: State::New,
            url,
            played_count: 0,
        }
    }
}