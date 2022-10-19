// use std::process::{Command, Stdio};
// use actix_web::web::Buf;
// use log::info;
// use rustube::url::Url;
// use crate::utils::tube_utils::parse_youtube_url;
// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub enum PlayerProcessError {
//     #[error("Rustube error")]
//     Rustube(#[from] rustube::Error),
//
//     #[error("Player error")]
//     Command(#[from] std::io::Error),
//
//     // #[error("Command error")]
//     // Command(#[from] rustube::Error),
//     // #[error("the data for key `{0}` is not available")]
//     // Redaction(String),
//     // #[error("invalid header (expected {expected:?}, found {found:?})")]
//     // InvalidHeader {
//     //     expected: String,
//     //     found: String,
//     // },
//     // #[error("unknown data store error")]
//     // Unknown,
// }
//
// static IINA_PATH: &str = "/opt/homebrew/bin/iina";
//
// pub async fn play_url(url: &str) -> Result<(), PlayerProcessError> {
//     let url = parse_youtube_url(url).await?;
//     println!("[IINA] Playing url={}", &url);
//     let cmd = Command::new(IINA_PATH)
//         .arg("--keep-running")
//         .arg(&url)
//         .arg("/dev/null")
//         .arg("--mpv-fullscreen")
//         .arg("--mpv-keep-open")
//         .arg("no")
//         .arg("--mpv-keep-open-pause")
//         .arg("no")
//
//         // .stdout(Stdio::null())
//         // .stderr(Stdio::null())
//         .spawn()
//         .expect("IINA command failed to start")
//         .wait()
//         .expect("IINA command failed to run");
//
//     println!("[IINA] Exit code={}, Finished playing url={}", cmd, url);
//
//     Ok(())
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {
//         play_url("/Users/eppe/Movies/projects/tennis_good_shots_30aug2022/render/game1.m4v");
//     }
// }
