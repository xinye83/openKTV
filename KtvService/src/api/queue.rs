use std::sync::Mutex;
use actix_web::{get, post, put, delete};
use actix_web::web::{Data, Json, Path, Query};
use log::info;
use crate::api::{ApiError, QueryParams};
use crate::api::song::SongIdRequest;
use crate::{ChildContainer, DBRepository};
use crate::model::queue::Queue;
use crate::utils::vlc_utils::play_url;


#[get("/queue")]
pub async fn get_q(ddb: Data<DBRepository>, params: Query<QueryParams>) -> Result<Json<Vec<Queue>>, ApiError> {
    let rtn = ddb.get_queue(params.0).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    };
}

#[put("/queue/next_song")]
pub async fn put_next_song(ddb: Data<DBRepository>, cc_data: Data<Mutex<ChildContainer>>) -> Result<Json<Vec<Queue>>, ApiError> {

    // next song requested
    let mut cc = cc_data.lock().unwrap();

    if cc.child.as_ref().is_some() {
        // previous vlc is running
        info!("killing child process for song ID ={}", cc.song_id);
        let child = cc.child.as_mut().unwrap();
        child.kill().expect("Unable to kill the child process");
        // remove from queue
        ddb.delete_song_from_q(cc.song_id.to_string()).await.unwrap();
    }

    let rtn = ddb.get_next_song().await.map_err(|e| ApiError::DbError(e))?;
    if let Some(queue) = rtn {
        let child = play_url(&queue.song_url).await.map_err(|_| ApiError::PlayerProcessError)?;
        //child.wait().expect("VLC command failed to run");

        info!("Playing {}", queue.song_name);
        cc.song_id = queue.song_id;
        cc.child = Some(child);

        let rtn = ddb.get_queue(QueryParams { page_num: Some(0), page_size: Some(1000) }).await;
        match rtn {
            Ok(id) => Ok(Json(id)),
            Err(err) => Err(ApiError::DbError(err))
        }
    } else {
        cc.song_id = 0;
        cc.child = None;
        Ok(Json(Vec::new()))
    }
}

// #[put("/queue/next_song")]
// pub async fn put_next_song(ddb: Data<DBRepository>, params: Query<QueryParams>) -> Result<Json<Vec<Queue>>, ApiError> {
//     let rtn = ddb.pop_song_from_queue(params.0).await;
//     return match rtn {
//         Ok(id) => Ok(Json(id)),
//         Err(err) => Err(ApiError::DbError(err))
//     };
// }


#[post("/queue/{song_id}")]
pub async fn post_song_to_q(ddb: Data<DBRepository>, path: Path<SongIdRequest>) -> Result<Json<u64>, ApiError> {
    let rtn = ddb.append_song_to_q(path.into_inner().song_id).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    };
}

#[put("/queue/{song_id}/prioritize")]
pub async fn put_prioritize_song(ddb: Data<DBRepository>, path: Path<SongIdRequest>, params: Query<QueryParams>) -> Result<Json<Vec<Queue>>, ApiError> {
    let rtn = ddb.prioritize_song_in_q(path.into_inner().song_id, false, params.0).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    };
}

#[put("/queue/{song_id}/deprioritize")]
pub async fn put_deprioritize_song(ddb: Data<DBRepository>, path: Path<SongIdRequest>, params: Query<QueryParams>) -> Result<Json<Vec<Queue>>, ApiError> {
    let rtn = ddb.prioritize_song_in_q(path.into_inner().song_id, true, params.0).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    };
}

#[delete("/queue/{song_id}")]
pub async fn delete_song_from_q(ddb: Data<DBRepository>, path: Path<SongIdRequest>) -> Result<Json<u64>, ApiError> {
    let rtn = ddb.delete_song_from_q(path.into_inner().song_id).await;
    return match rtn {
        Ok(id) => Ok(Json(id)),
        Err(err) => Err(ApiError::DbError(err))
    };
}
