use log::info;
use crate::api::QueryParams;
use crate::model::queue::Queue;
use crate::repo::{create_pagination_query, DBRepository};


impl DBRepository {
    pub async fn get_queue(&self, params: QueryParams) -> Result<Vec<Queue>, sqlx::Error> {
        //In MySQL NULL values are considered lower in order than any non-NULL value, except if a - (minus) character is added before the column name while sorting.
        let query_str = format!("\
SELECT q.*, a.name AS artist_name, s.id AS song_id, s.name AS song_name
FROM queue q
LEFT JOIN song s ON s.id = q.song_id
LEFT JOIN artist a ON s.artist_id = a.id
ORDER BY -q.prioritized_at DESC, q.created_at ASC {}", create_pagination_query(&params));
        let result = sqlx::query_as::<_, Queue>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;
        Ok(result)
    }

    pub async fn next_song(&self, params: QueryParams) -> Result<Vec<Queue>, sqlx::Error> {
        let current_queue = self.get_queue(QueryParams { page_num: Some(0), page_size: Some(1) }).await?;
        if current_queue.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }
        let id = current_queue.first().unwrap().id;
        let query_str = format!("DELETE FROM queue WHERE id = {}", &id);
        let result = sqlx::query(query_str.as_str())
            .execute(&self.pool)
            .await?;
        info!("Deleted {}. {:?}", id, result);

        let updated_queue = self.get_queue(params).await?;
        Ok(updated_queue)
    }

    pub async fn append_song_to_q(&self, song_id: String) -> Result<u64, sqlx::Error> {
        // validate song existence
        self.get_song_by_id(song_id.clone()).await?;

        let result = sqlx::query("\
INSERT INTO queue (song_id, created_at, prioritized_at)
VALUES (?, NOW(), NULL)
")
            .bind(song_id)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_id())
    }

    pub async fn prioritize_song_in_q(&self, song_id: String, is_deprioritize: bool, params: QueryParams) -> Result<Vec<Queue>, sqlx::Error> {
        // validate song existence
        self.get_song_by_id(song_id.clone()).await?;
        let prioritize_value = if is_deprioritize { "NULL" } else { "NOW()" };
        let query_str = format!("\
UPDATE queue
SET prioritized_at = {}
WHERE song_id = {}", prioritize_value, song_id);
        let result = sqlx::query(query_str.as_str())
            .execute(&self.pool)
            .await?;
        info!("{:?}", result);
        let updated_queue = self.get_queue(params).await?;
        Ok(updated_queue)
    }

    pub async fn delete_song_from_q(&self, song_id: String) -> Result<u64, sqlx::Error> {
        let query_str = format!("\
DELETE FROM queue
WHERE song_id = {}", song_id);
        let result = sqlx::query(query_str.as_str())
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}