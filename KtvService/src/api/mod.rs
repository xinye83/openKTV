pub mod song;
pub mod artist;

use actix_web::{
    HttpResponse,
    ResponseError,
    http::StatusCode,
    http::header::ContentType,
    web::Json
};
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

pub fn match_results<T>(rtn: Result<Vec<T>, Error>, params: QueryParams) -> Result<Json<ApiResponse<T>>, ApiError> {
    return match rtn {
        Ok(it) => Ok(Json(ApiResponse {
            page_num: params.page_num,
            page_size: params.page_size,
            items: Box::new(it),
        })),
        Err(err) => Err(ApiError::DbError(err))
    }
}
