#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

mod discord;
mod routes;
mod util;
use util::Settings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let cfg = Settings::get();

    tokio::spawn(async { discord::init().await.unwrap() });
    HttpServer::new(move || {
        let cfg = Settings::get();
        App::new()
            .data(cfg)
            .wrap(Logger::default())
            .service(web::scope("/api").configure(routes::config))
    })
    .bind((cfg.http.bind, cfg.http.port))?
    .run()
    .await
}
