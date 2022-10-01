use std::fmt::{Display, Formatter};
use actix_web::{get, post, put, error::ResponseError, web::Path, web::Json, web::Data, HttpResponse, http::{header::ContentType, StatusCode}, web};
use aws_sdk_dynamodb::error::ScanError;
use aws_sdk_dynamodb::types::SdkError;
use serde::{Deserialize, Serialize};

use crate::model::song::Song;
use crate::repo::ddb::{DdbError, DdbErrorExt, DDBRepository};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "internal DB error")]
    DbError(DdbError),

    // #[display(fmt = "bad request")]
    // BadClientData(SdkError<ScanError>),
    //
    // #[display(fmt = "timeout")]
    // Timeout,
}


impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/song/{id}")]
pub async fn get_song_by_id(song: Path<Song>) -> Json<String> {

    return Json("hello world".to_string());
}

#[get("/songs")]
pub async fn get_all_songs<'a>(ddb: Data<DDBRepository>) -> Result<Json<Vec<Song>>, ApiError<'a>> {
    let rtn = ddb.get_songs().await.map_err(|e| { e.to_api_error() })?;
    Ok(Json(rtn))
}

