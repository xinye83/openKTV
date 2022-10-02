mod api;
mod model;
mod repo;
mod utils;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use api::song::*;
use utils::consts::*;
use crate::repo::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let shared_config = aws_config::load_from_env().await;

    HttpServer::new(move || {
        let ddb = DDBRepository::init(KTV_SONGS_TABLE_NAME.to_string(), shared_config.clone());
        let ddb_data = Data::new(ddb);
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_all_songs)
            .service(put_song)
            .service(query_songs)

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
