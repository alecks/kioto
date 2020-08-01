#[macro_use]
extern crate log;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use std::sync::Arc;
use tokio;

mod discord;
mod settings;
use settings::Settings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    // FIXME: the way cfg is being passed around is horrible, im tired thanks
    let cfg = Arc::new(Settings::new().expect("Couldn't load config"));

    tokio::spawn(async { discord::init().await.unwrap() });
    let captured_cfg = cfg.clone();
    HttpServer::new(move || {
        App::new()
            .data(captured_cfg.clone())
            .wrap(Logger::default())
        //.service(web::scope("/api/v1").configure(routes::version_one::config))
    })
    .bind(&cfg.http.address)?
    .run()
    .await
}
