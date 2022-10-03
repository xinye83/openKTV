mod api;
mod model;
mod repo;
mod utils;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use sqlx::mysql::MySqlPoolOptions;
use api::song::*;
use repo::db::*;
use utils::consts::*;
use crate::repo::db::DBRepository;

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
            .service(get_all_artists)
            .service(put_artist)
            .service(query_artists)
            .service(put_song)
            .service(query_songs)

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
