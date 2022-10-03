
use actix_web::web::{Json, Query};
use log::{debug, error, info};
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlQueryResult;
use crate::ApiError::DbError;
use crate::model::song::{Artist, Song};
use crate::SongApi;

#[derive(Clone)]
pub struct DBRepository {
    pool: Pool<MySql>,
}


#[derive(thiserror::Error, Debug)]
pub enum DdbError {
    #[error("Item not found in DB")]
    ItemNotFoundItem(String)
}


impl DBRepository {
    pub async fn init(pool: Pool<MySql>) -> Result<DBRepository, sqlx::Error> {
        sqlx::query("
CREATE TABLE IF NOT EXISTS artist
(
    id          BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name`      VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    region      VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
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

    // pub async fn get_songs(&self) -> Result<Vec<Song>, DdbError> {
    //     let request = self.client.scan()
    //         .table_name(&self.table_name);
    //
    //     let response = request.send().await?;
    //
    //     let songs:Vec<Song> = from_items(response.items.unwrap())?;
    //     Ok(songs)
    // }

    // pub async fn put_song(&self, song: &Song) -> Result<PutItemOutput, DdbError> {
    //
    //     let row: (u64,) = sqlx::query_as("INSERT INTO ")
    //     let item = to_item(song)?;
    //     let request = self.client.put_item()
    //         .table_name(&self.table_name)
    //         .set_item(Some(item));
    //
    //     let response = request.send()
    //         .await?;
    //
    //     Ok(response)
    // }
    //
    pub async fn insert_artist(&self, name: String, region: String) -> Result<u64, sqlx::Error> {
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

    pub async fn get_all_artists(&self) -> Result<Vec<Artist>, sqlx::Error> {
        let result = sqlx::query_as::<_, Artist>("SELECT * FROM artist")
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn query_artists_by_name(&self, name: String) -> Result<Vec<Artist>, sqlx::Error> {
        let query_str = format!("SELECT * FROM artist WHERE name LIKE '%{}%'", name);
        let result = sqlx::query_as::<_, Artist>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;

        Ok(result)

    }

    // pub async fn query_songs(&self, query: Json<SongApi>) -> Result<Vec<Song>, DdbError> {
    //     // if query has uuid, run query by ID and return
    //     if let Some(uuid) = query.uuid.clone() {
    //         let item = self.client
    //             .get_item()
    //             .table_name(&self.table_name)
    //             .key("uuid", AttributeValue::S(uuid.clone()))
    //             .send()
    //             .await?;
    //
    //         return match item.item {
    //             None => Err(DdbError::ItemNotFoundItem(uuid)),
    //             Some(it) => {
    //                 let song: Song = from_item(it)?;
    //                 Ok(vec![song])
    //             },
    //         }
    //     }
    //
    //     let mut condition: Vec<String> = Vec::new();
    //
    //     // Declare all of the expression inputs for a query call
    //     let mut expression_attribute_values = HashMap::new();
    //
    //     if let Some(name) = query.name.clone() {
    //         condition.push("name = :name".to_string());
    //         expression_attribute_values.insert(":name".to_string(), to_attribute_value(name)?);
    //     }
    //
    //     if let Some(artist) = query.artist.clone() {
    //         condition.push("artist = :artist".to_string());
    //         expression_attribute_values.insert(":artist".to_string(), to_attribute_value(artist)?);
    //     }
    //
    //     let key_condition_exp = condition.join(" AND ");
    //
    //     info!("Query filter_expression={}", &key_condition_exp);
    //
    //     let request = self.client.
    //         scan()
    //         .table_name(&self.table_name)
    //         .filter_expression(key_condition_exp)
    //         .set_expression_attribute_values(Some(expression_attribute_values))
    //         .send()
    //         .await?;
    //
    //     let songs:Vec<Song> = from_items(request.items.unwrap())?;
    //     Ok(songs)
    // }
}