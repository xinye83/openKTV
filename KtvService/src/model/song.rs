use chrono::{DateTime, Utc};
use serde::{
    Deserialize,
    Serialize
};
use crate::model::artist::Artist;


#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Song {
    pub id: u64,
    pub name: String,
    #[sqlx(flatten)]
    pub artist: Artist,
    //pub state: State,
    pub url: String,
    pub is_queued: bool,
    //pub played_count: usize,
    //#[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

}
