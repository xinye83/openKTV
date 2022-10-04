use actix_web::{post, put};
use actix_web::web::{Data, Json, Query};
use crate::api::{ApiError, ApiResponse, match_results, QueryParams};
use crate::DBRepository;
use crate::utils::OptionUtil;
use serde::{Deserialize, Serialize};
use crate::model::artist::Artist;

#[derive(Serialize, Deserialize)]
pub struct ArtistRequest {
    pub name: Option<String>,
    pub region: Option<String>,
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
pub async fn query_artists(ddb: Data<DBRepository>, query: Json<ArtistRequest>, params: Query<QueryParams>) -> Result<Json<ApiResponse<Artist>>, ApiError> {
    let rtn = ddb.query_artists(query.0, &params.0).await;
    return match_results(rtn, params.0)

}