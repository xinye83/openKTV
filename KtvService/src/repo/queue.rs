use log::info;
use crate::model::queue::Queue;
use crate::repo::{create_pagination_query_str, DBRepository};

impl DBRepository {
    pub async fn get_queue(&self) -> Result<Vec<Queue>, sqlx::Error> {
        //In MySQL NULL values are considered lower in order than any non-NULL value, except if a - (minus) character is added before the column name while sorting.
        let query_str = "\
SELECT q.*, a.name AS artist_name, a.*, s.*
FROM queue q
LEFT JOIN song s ON s.id = q.song_id
LEFT JOIN artist a ON s.artist_id = a.id
ORDER BY -q.prioritized_at DESC, q.created_at ASC";
        let result = sqlx::query_as::<_, Queue>(query_str)
            .fetch_all(&self.pool)
            .await?;
        Ok(result)
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

    pub async fn prioritize_song_in_q(&self, song_id: String, is_deprioritize: bool) -> Result<Vec<Queue>, sqlx::Error> {
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
        let updated_queue = self.get_queue().await?;
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