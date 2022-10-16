use chrono::{DateTime, Utc};
use serde::{
    Deserialize,
    Serialize
};


#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Artist {
    pub id: u64,
    #[sqlx(rename = "artist_name")]
    pub name: String,
    pub region: String,
    //#[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
