mod artist;
mod song;

use sqlx::{MySql, Pool};


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

}