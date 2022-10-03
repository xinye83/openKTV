use sqlx::{MySql, Pool};
use crate::model::song::{Artist, Song};
use crate::{ArtistRequest, QueryParams, SongRequest};

#[derive(Clone)]
pub struct DBRepository {
    pool: Pool<MySql>,
}

pub fn create_pagination_query_str(page_num: Option<usize>, page_size: Option<usize>) -> String {
    let page_num = page_num.unwrap_or(0);
    let page_size = page_size.unwrap_or(50);
    let offset = page_num * page_size;
    format!(" LIMIT {} OFFSET {}", page_size, offset)
}


impl DBRepository {
    pub async fn init(pool: Pool<MySql>) -> Result<DBRepository, sqlx::Error> {
        sqlx::query("
CREATE TABLE IF NOT EXISTS artist
(
    id          BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name`      VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    region      VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_artist UNIQUE (`name`)
);").execute(&pool).await?;

        sqlx::query("
CREATE TABLE IF NOT EXISTS song
(
    id          BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name`      VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    artist_id   BIGINT UNSIGNED NOT NULL,
    url         TEXT NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uniq_song UNIQUE (`name`, artist_id),
    FOREIGN KEY (artist_id)
      REFERENCES artist(id)
      ON DELETE CASCADE
);").execute(&pool).await?;

        Ok(DBRepository {
            pool
        })
    }

    pub async fn insert_artist(&self, name: &String, region: &Option<String>) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("
INSERT INTO artist (name, region)
VALUES (?, ?)
")
            .bind(name)
            .bind(region)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_id())
    }

    pub async fn get_all_artists(&self, query: &QueryParams) -> Result<Vec<Artist>, sqlx::Error> {
        let query_str = format!("SELECT * FROM artist {}", create_pagination_query_str(query.page_num, query.page_size));
        let result = sqlx::query_as::<_, Artist>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn query_artists(&self, request: ArtistRequest, query: &QueryParams) -> Result<Vec<Artist>, sqlx::Error> {
        let mut where_causes: Vec<(&str, String)> = Vec::new();
        if let Some(name) = request.name {
            where_causes.push(("name", name))
        }
        if let Some(region) = request.region {
            where_causes.push(("region", region))
        }
        let where_cause_str = where_causes.iter()
            .map(|it| format!("{} LIKE '%{}%'", it.0, it.1))
            .collect::<Vec<_>>()
            .join(" AND ");

        let query_str = format!("SELECT a.id, a.name AS artist_name, a.region FROM artist a WHERE {} {}", where_cause_str, create_pagination_query_str(query.page_num, query.page_size));
        let result = sqlx::query_as::<_, Artist>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn insert_song(&self, request: SongRequest) -> Result<u64, sqlx::Error> {
        // find artist first
        let artist_request = ArtistRequest {
            name: request.name.clone(),
            region: request.region.clone(),
        };
        let artists = self.query_artists(artist_request, &QueryParams { page_num: None, page_size: None }).await?;

        let artist_id = if artists.is_empty() {
            self.insert_artist(request.name.as_ref().unwrap(), &request.region).await?
        } else {
            artists.get(0).unwrap().id
        };


        let result = sqlx::query("
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
SELECT s.id, s.name, s.url, a.name AS artist_name, a.region
FROM song s
LEFT JOIN artist a ON s.artist_id = a.id
WHERE s.name LIKE '%{}%' {}", body.name.unwrap(), create_pagination_query_str(query.page_num, query.page_size));
        let result = sqlx::query_as::<_, Song>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }
}