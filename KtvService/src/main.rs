mod api;
mod model;
mod repo;
mod utils;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use sqlx::mysql::MySqlPoolOptions;
use api::artist::{put_artist, query_artists};
use repo::DBRepository;
use crate::api::queue::{delete_song_from_q, get_q, post_song_to_q, put_deprioritize_song, put_prioritize_song};
use crate::api::song::{get_song_by_id, put_song, query_songs};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    //std::env::set_var("DATABASE_URL", "mysql://root:sys_admin_123@mysql/ktv" );
    env_logger::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect("mysql://root:sys_admin_123@localhost:3306/ktv?useUnicode=true")
        .await
        .expect("DB connection failed.");

    let ddb = DBRepository::init(pool)
        .await
        .expect("DB failed to init.");

    HttpServer::new(move || {
        let ddb_data = Data::new(ddb.clone());
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(put_artist)
            .service(query_artists)
            .service(get_song_by_id)
            .service(put_song)
            .service(query_songs)
            .service(get_q)
            .service(post_song_to_q)
            .service(put_prioritize_song)
            .service(put_deprioritize_song)
            .service(delete_song_from_q)

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
