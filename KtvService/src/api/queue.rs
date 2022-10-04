use actix_web::{get, post, put, delete};
use actix_web::web::{Data, Json, Path, Query};
use crate::model::song::Song;
use serde::{Deserialize, Serialize};
use crate::api::{ApiError, ApiResponse, match_results, QueryParams};
use crate::api::song::SongIdRequest;
use crate::DBRepository;
use crate::model::queue::Queue;


#[get("/queue")]
pub async fn get_q(ddb: Data<DBRepository>) -> Result<Json<Vec<Queue>>, ApiError> {
    let rtn = ddb.get_queue().await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

#[post("/queue/{song_id}")]
pub async fn post_song_to_q(ddb: Data<DBRepository>, path: Path<SongIdRequest>) -> Result<Json<u64>, ApiError> {
    let rtn = ddb.append_song_to_q(path.into_inner().song_id).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

#[put("/queue/{song_id}/prioritize")]
pub async fn put_prioritize_song(ddb: Data<DBRepository>, path: Path<SongIdRequest>) -> Result<Json<Vec<Queue>>, ApiError> {
    let rtn = ddb.prioritize_song_in_q(path.into_inner().song_id, false).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

#[put("/queue/{song_id}/deprioritize")]
pub async fn put_deprioritize_song(ddb: Data<DBRepository>, path: Path<SongIdRequest>) -> Result<Json<Vec<Queue>>, ApiError> {
    let rtn = ddb.prioritize_song_in_q(path.into_inner().song_id, true).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

#[delete("/queue/{song_id}")]
pub async fn delete_song_from_q(ddb: Data<DBRepository>, path: Path<SongIdRequest>) -> Result<Json<u64>, ApiError> {
    let rtn = ddb.delete_song_from_q(path.into_inner().song_id).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    }
}
