use chrono::{DateTime, Utc};
use serde::{
    Deserialize,
    Serialize
};
use crate::model::song::Song;


#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Queue {
    pub id: u64,
    #[sqlx(flatten)]
    pub song: Song,
    pub created_at: DateTime<Utc>,
    pub prioritized_at: Option<DateTime<Utc>>,
}
