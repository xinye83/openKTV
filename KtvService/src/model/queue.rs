use chrono::{DateTime, Utc};
use serde::{
    Deserialize,
    Serialize
};


#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Queue {
    pub id: u64,
    pub song_id: u64,
    pub song_name: String,
    pub song_url: String,
    pub artist_name: String,
    pub created_at: DateTime<Utc>,
    pub prioritized_at: Option<DateTime<Utc>>,
}
