use sqlx::Row;
use crate::api::artist::ArtistRequest;
use crate::api::QueryParams;
use crate::api::song::SongRequest;
use crate::model::song::Song;
use crate::repo::{create_pagination_query, DBRepository};

static SONG_SELECT_FIELDS: &str = "a.name AS artist_name, a.*, s.*, IF(EXISTS(SELECT 1 FROM queue WHERE queue.song_id = s.id), true, false) as is_queued";

impl DBRepository {
    pub async fn insert_song(&self, request: SongRequest) -> Result<u64, sqlx::Error> {
        // find artist first
        let artist_request = ArtistRequest {
            name: Some(request.artist.clone()),
            region: request.region.clone(),
        };
        let artists = self.query_artists(artist_request, &QueryParams { page_num: None, page_size: None }).await?;

        let artist_id = if artists.is_empty() {
            self.insert_artist(&request.artist, &request.region).await?
        } else {
            artists.get(0).unwrap().id
        };

        let result = sqlx::query("\
INSERT INTO song (name, url, artist_id)
VALUES (?, ?, ?)
")
            .bind(request.name)
            .bind(request.url.unwrap())
            .bind(artist_id)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_id())
    }

    pub async fn count_total_songs(&self, song_name: &str, artist: &str) -> Result<i64, sqlx::Error> {
        let count_query_str = format!("\
SELECT COUNT(*)
FROM song s
LEFT JOIN artist a ON s.artist_id = a.id
WHERE s.name LIKE '%{}%' AND a.name LIKE '%{}%' ", song_name, artist);
        let count: i64 = sqlx::query(count_query_str.as_str())
            .fetch_one(&self.pool)
            .await?
            .try_get(0)?;
        Ok(count)
    }

    pub async fn query_songs(&self, body: SongRequest, query: &QueryParams) -> Result<(Vec<Song>, i64), sqlx::Error> {

        let count = self.count_total_songs(&body.name, &body.artist).await?;

        let query_str = format!("\
SELECT {}
FROM song s
LEFT JOIN artist a ON s.artist_id = a.id
WHERE s.name LIKE '%{}%' AND a.name LIKE '%{}%' {}", SONG_SELECT_FIELDS, &body.name, &body.artist, create_pagination_query(query));
        let result = sqlx::query_as::<_, Song>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;

        Ok((result, count))
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