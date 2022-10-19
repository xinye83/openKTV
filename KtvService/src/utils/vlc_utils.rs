use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use actix_web::web::Buf;
use log::{info, logger};
use rustube::url::Url;
use crate::utils::tube_utils::parse_youtube_url;
use thiserror::Error;
use crate::DBRepository;

#[derive(Error, Debug)]
pub enum PlayerProcessError {
    #[error("Rustube error")]
    Rustube(#[from] rustube::Error),

    #[error("Player error")]
    Command(#[from] std::io::Error),

    // #[error("Command error")]
    // Command(#[from] rustube::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
}

pub struct ChildContainer {
    pub song_id: u64,
    pub child: Option<Child>,
}

static VLC_PATH: &str = "/opt/homebrew/bin/vlc";

pub async fn play_url(ddb: &DBRepository, url: &str, song_id: u64, cc_data: Arc<Mutex<ChildContainer>>) -> Result<(), PlayerProcessError> {
    let mut cc = cc_data.lock().unwrap();

    if cc.child.as_ref().is_some() { // previous vlc is running
        info!("killing child process for song ID ={}", cc.song_id);
        let child = cc.child.as_mut().unwrap();
        child.kill().expect("Should kill the child process");
        // remove from queue
        ddb.delete_song_from_q(cc.song_id.to_string()).await.unwrap();
    }

    let url = parse_youtube_url(url).await?;
    println!("[VLC] Playing url={}", &url);
    let child = Command::new(VLC_PATH)
        .arg(&url)
        .arg("--fullscreen")
        .arg("--play-and-exit")

        // .stdout(Stdio::null())
        // .stderr(Stdio::null())
        .spawn()
        .expect("VLC command failed to start");

    cc.child = Some(child);
    cc.song_id = song_id;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test() {
    //     play_url("/Users/eppe/Movies/projects/tennis_good_shots_30aug2022/render/game1.m4v");
    // }
}
