use rustube::{Error, Id, Video};

pub async fn parse_youtube_url(url: &str) -> Result<String, Error> {
    let id = Id::from_raw(url)?;
    let video = Video::from_id(id.into_owned()).await?;
    let stream = video
        .best_quality()
        .ok_or(Error::NoStreams)
        .unwrap();
    Ok(stream.signature_cipher.url.to_string())
}

#[cfg(test)]
mod tests {
    use rustube::{Error, Video};
    use crate::utils::iina_utils::play_url;
    use super::*;

    #[actix_rt::test]
    async fn factorial_of_0() -> Result<(), Error> {
        parse_youtube_url("https://www.youtube.com/watch?v=DrtlnWn8y_U");

        Ok(())
    }
}
