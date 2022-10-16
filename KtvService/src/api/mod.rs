pub mod song;
pub mod artist;
pub mod queue;

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

    #[display(fmt = "Bad CSV file or format")]
    CsvReadError(csv::Error),

    #[display(fmt = "Bad request")]
    BadClientData,

    #[display(fmt = "Internal process error")]
    PlayerProcessError
    //
    // #[display(fmt = "timeout")]
    // Timeout,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DbError(_) => StatusCode::BAD_REQUEST,
            ApiError::CsvReadError(_) => StatusCode::BAD_REQUEST,
            ApiError::BadClientData => StatusCode::BAD_REQUEST,
            ApiError::PlayerProcessError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = match self {
            ApiError::DbError(err) => {
                format!("{:?}", err)
            },
            _ => self.to_string()
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
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
    pub total_count: Option<i64>,
    pub items: Box<Vec<T>>,
}

pub fn match_results<T>(rtn: Result<Vec<T>, Error>, params: QueryParams) -> Result<Json<ApiResponse<T>>, ApiError> {
    return match rtn {
        Ok(it) => Ok(Json(ApiResponse {
            page_num: params.page_num,
            page_size: params.page_size,
            total_count: None,
            items: Box::new(it),
        })),
        Err(err) => Err(ApiError::DbError(err))
    }
}

pub fn match_results_total<T>(rtn: Result<(Vec<T>, i64), Error>, params: QueryParams) -> Result<Json<ApiResponse<T>>, ApiError> {
    return match rtn {
        Ok(it) => Ok(Json(ApiResponse {
            page_num: params.page_num,
            page_size: params.page_size,
            total_count: Some(it.1),
            items: Box::new(it.0),
        })),
        Err(err) => Err(ApiError::DbError(err))
    }
}

// pub trait Result {
//     fn api_unwrap(&self) -> Result<T, ApiError>
// }
