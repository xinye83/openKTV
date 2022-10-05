use actix_web::{get, post, put};
use actix_web::web::{Data, Json, Path, Query};
use crate::model::song::Song;
use serde::{Deserialize, Serialize};
use crate::api::{ApiError, ApiResponse, match_results, QueryParams};
use crate::DBRepository;

#[derive(Serialize, Deserialize)]
pub struct SongIdRequest {
    pub song_id: String
}

#[derive(Serialize, Deserialize)]
pub struct SongRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub artist: Option<String>,
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ListRequest {
    pub path: String
}


// song
#[get("/song/{song_id}")]
pub async fn get_song_by_id(ddb: Data<DBRepository>, path: Path<SongIdRequest>) -> Result<Json<Song>, ApiError> {
    let rtn = ddb.get_song_by_id(path.into_inner().song_id).await;
    return match rtn {
        Ok(s) => Ok(Json(s)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

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

/// Insert a list of songs in CSV format into the database, return an error or a list of song IDs if success.
#[put("/list")]
pub async fn put_list(ddb: Data<DBRepository>, path: Json<ListRequest>) -> Result<Json<Vec<u64>>, ApiError> {
    let mut reader = match csv::Reader::from_path(path.into_inner().path) {
        Ok(r) => r,
        Err(err) => return Err(ApiError::CsvReadError(err)),
    };

    let mut ids: Vec<u64> = Vec::new();

    for item in reader.deserialize() {
        let item: SongRequest = match item {
            Ok(item) => item,
            Err(err) => return Err(ApiError::CsvReadError(err)),
        };

        let rtn = ddb.insert_song(item).await;

        match rtn {
            Ok(id) => ids.push(id),
            Err(_) => continue,
        }
    }

    return Ok(Json(ids))
}
