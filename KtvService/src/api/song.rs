use actix_web::{get, post, put, error::ResponseError, web::Path, web::Json, web::Data, HttpResponse, http::{header::ContentType, StatusCode}, web};
use actix_web::web::Query;
use crate::model::song::Song;
use crate::repo::ddb::{DdbError, DDBRepository};
use crate::utils::*;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Internal DB error")]
    DbError(DdbError),

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
pub struct SongApi {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub artist: Option<String>,
    pub url: Option<String>,
}


#[get("/song/{id}")]
pub async fn get_song_by_id(song: Path<Song>) -> Json<String> {

    return Json("hello world".to_string());
}

#[get("/all_songs")]
pub async fn get_all_songs(ddb: Data<DDBRepository>) -> Result<Json<Vec<Song>>, ApiError> {
    let rtn = ddb.get_songs().await;
    return match rtn {
        Ok(it) => Ok(Json(it)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

#[put("/song")]
pub async fn put_song(ddb: Data<DDBRepository>, payload: Json<SongApi>) -> Result<Json<String>, ApiError> {
    let song = Song::new_youtube(payload.name.guard()?, payload.artist.guard()?, payload.url.guard()?);
    let rtn = ddb.put_song(&song).await;
    return match rtn {
        Ok(_) => Ok(Json(song.uuid)),
        Err(err) => Err(ApiError::DbError(err))
    }
}

#[get("/songs")]
pub async fn query_songs(ddb: Data<DDBRepository>, query: Json<SongApi>) -> Result<Json<Vec<Song>>, ApiError> {
    let rtn = ddb.query_songs(query).await;
    return match rtn {
        Ok(it) => Ok(Json(it)),
        Err(err) => Err(ApiError::DbError(err))
    }
}