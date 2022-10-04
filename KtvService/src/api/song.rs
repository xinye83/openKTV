use actix_web::{get, post, put, error::ResponseError, web::Path, web::Data, HttpResponse, http::{header::ContentType, StatusCode}, web};
use actix_web::web::{Json, Query};
use crate::model::song::{Artist, Song};
use crate::repo::db::DBRepository;
use crate::utils::*;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use sqlx::{Error};


#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Internal DB error")]
    DbError(sqlx::Error),

    #[display(fmt = "Bad request")]
    BadClientData,
    //
    // #[display(fmt = "timeout")]
    // Timeout,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadClientData => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct SongRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub artist: Option<String>,
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistRequest {
    pub name: Option<String>,
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    pub page_num: Option<usize>, // 0-based
    pub page_size: Option<usize>,
    //pub include_total_count: Option<bool>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub page_num: Option<usize>,
    pub page_size: Option<usize>,
    pub items: Box<Vec<T>>,
}

// artist
#[put("/artist")]
pub async fn put_artist(ddb: Data<DBRepository>, payload: Json<ArtistRequest>) -> Result<Json<u64>, ApiError> {
    let rtn = ddb.insert_artist(&payload.name.guard()?, &payload.region).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

#[post("/artists")]
pub async fn query_artists(ddb: Data<DBRepository>, query: web::Json<ArtistRequest>, params: Query<QueryParams>) -> Result<Json<ApiResponse<Artist>>, ApiError> {
    let rtn = ddb.query_artists(query.0, &params.0).await;
    return match_results(rtn, params.0)

}

// song
#[put("/song")]
pub async fn put_song(ddb: Data<DBRepository>, payload: Json<SongRequest>) -> Result<Json<u64>, ApiError> {
    //let song = Song::new_youtube(payload.name.guard()?, payload.artist.guard()?, payload.url.guard()?);
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

fn match_results<T>(rtn: Result<Vec<T>, Error>, params: QueryParams) -> Result<Json<ApiResponse<T>>, ApiError> {
    return match rtn {
        Ok(it) => Ok(Json(ApiResponse {
            page_num: params.page_num,
            page_size: params.page_size,
            items: Box::new(it),
        })),
        Err(err) => Err(ApiError::DbError(err))
    }
}
