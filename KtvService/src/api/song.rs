use actix_web::{post, put};
use actix_web::web::{Data, Json, Query};
use crate::model::song::Song;
use serde::{Deserialize, Serialize};
use crate::api::{ApiError, ApiResponse, match_results, QueryParams};
use crate::DBRepository;


#[derive(Serialize, Deserialize)]
pub struct SongRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub artist: Option<String>,
    pub region: Option<String>,
}

// song
#[put("/song")]
pub async fn put_song(ddb: Data<DBRepository>, payload: Json<SongRequest>) -> Result<Json<u64>, ApiError> {
    let rtn = ddb.insert_song(payload.0).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

#[post("/songs")]
pub async fn query_songs(ddb: Data<DBRepository>, query: Json<SongRequest>, params: Query<QueryParams>) -> Result<Json<ApiResponse<Song>>, ApiError> {
    let rtn = ddb.query_songs(query.0, &params.0).await;
    return match_results(rtn, params.0)
}

