use std::io::{LineWriter, Write};
use actix_web::{get, HttpRequest, HttpResponse, post, put};
use actix_web::web::{Data, Json, Path, Query};
use crate::model::song::Song;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::api::{ApiError, ApiResponse, match_results, match_results_total, QueryParams};
use crate::DBRepository;
use tempfile::{NamedTempFile, tempfile};

#[derive(Serialize, Deserialize)]
pub struct SongIdRequest {
    pub song_id: String
}

#[derive(Serialize, Deserialize)]
pub struct SongRequest {
    pub name: String,
    pub artist: String,
    pub url: Option<String>,
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
    return match_results_total(rtn, params.0)
}


/// Import a list of songs in CSV format into the database, return an error or a list of song IDs if success.
#[post("/import")]
pub async fn post_songs_import(ddb: Data<DBRepository>, path: Json<ListRequest>) -> Result<Json<Vec<u64>>, ApiError> {
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

#[get("/export")]
pub async fn get_songs_export(req: HttpRequest, ddb: Data<DBRepository>) -> Result<HttpResponse, ApiError> {
    // Create a file inside of `std::env::temp_dir()`.
    let file = NamedTempFile::new().unwrap();

    let mut buffer = LineWriter::new(&file);

    let song_request = SongRequest {
        name: "".to_string(),
        artist: "".to_string(),
        url: None,
        region: None,
    };

    let query = QueryParams {
        page_num: Some(0),
        page_size: Some(10000),
    };

    let songs_results = ddb.query_songs(song_request, &query).await.map_err(|e| ApiError::DbError(e))?;

    for song in songs_results.0 {

        let line = json!(song).to_string();
        let str = line.as_bytes();
        buffer.write(str).unwrap();
    }

    let named_file = actix_files::NamedFile::open_async(&file.path()).await.unwrap();

    Ok(named_file.into_response(&req))
}