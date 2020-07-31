#[macro_use]
extern crate log;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use tokio;

mod discord;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    tokio::spawn(async { discord::init().await.unwrap() });
    HttpServer::new(|| {
        App::new().wrap(Logger::default())
        //.service(web::scope("/api/v1").configure(routes::version_one::config))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
