use crate::api::artist::ArtistRequest;
use crate::api::QueryParams;
use crate::api::song::SongRequest;
use crate::model::song::Song;
use crate::repo::{create_pagination_query_str, DBRepository};

static SONG_SELECT_FIELDS: &str = "a.name AS artist_name, a.*, s.*";

impl DBRepository {
    pub async fn insert_song(&self, request: SongRequest) -> Result<u64, sqlx::Error> {
        // find artist first
        let artist_request = ArtistRequest {
            name: request.artist.clone(),
            region: request.region.clone(),
        };
        let artists = self.query_artists(artist_request, &QueryParams { page_num: None, page_size: None }).await?;

        let artist_id = if artists.is_empty() {
            self.insert_artist(request.artist.as_ref().unwrap(), &request.region).await?
        } else {
            artists.get(0).unwrap().id
        };


        let result = sqlx::query("\
INSERT INTO song (name, url, artist_id)
VALUES (?, ?, ?)
")
            .bind(request.name.unwrap())
            .bind(request.url.unwrap())
            .bind(artist_id)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_id())
    }

    pub async fn query_songs(&self, body: SongRequest, query: &QueryParams) -> Result<Vec<Song>, sqlx::Error> {
        let query_str = format!("\
SELECT {}
FROM song s
LEFT JOIN artist a ON s.artist_id = a.id
WHERE s.name LIKE '%{}%' {}", SONG_SELECT_FIELDS, body.name.unwrap(), create_pagination_query_str(query.page_num, query.page_size));
        let result = sqlx::query_as::<_, Song>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn get_song_by_id(&self, id: String) -> Result<Song, sqlx::Error> {
        let query_str = format!("\
SELECT {}
FROM song s
LEFT JOIN artist a ON s.artist_id = a.id
WHERE s.id = {}
", SONG_SELECT_FIELDS, id);
        let result = sqlx::query_as::<_, Song>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;
        if result.is_empty() {
            return Err(sqlx::Error::RowNotFound)
        }
        Ok(result.first().unwrap().clone())
    }

}