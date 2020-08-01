#[macro_use]
extern crate lazy_static;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod api;
mod discord;
mod routes;
mod util;
use util::Settings;

use std::sync::Arc;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let cfg = Settings::get();
    let manager = ConnectionManager::<PgConnection>::new(cfg.db.url);
    let pool = Arc::new(
        r2d2::Pool::builder()
            .build(manager)
            .expect("Couldn't create pool"),
    );

    {
        let pool = pool.clone();
        tokio::spawn(async { discord::init(pool).await.unwrap() });
    }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .configure(routes::api::config)
    })
    .bind((cfg.http.bind, cfg.http.port))?
    .run()
    .await
}
