#[macro_use]
extern crate lazy_static;
use actix_session::CookieSession;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod api;
mod discord;
mod routes;
mod util;
use util::{AppState, Settings};

use std::sync::Arc;

use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

const AUTH_URL: &str = "https://discord.com/api/oauth2/authorize";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";

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
        let auth_url = AuthUrl::new(AUTH_URL.to_string()).unwrap();
        let token_url = TokenUrl::new(TOKEN_URL.to_string()).unwrap();

        let cfg = Settings::get();
        let discord_client_id = ClientId::new(cfg.bot.client_id);
        let discord_client_secret = ClientSecret::new(cfg.bot.client_secret);

        let oauth_client = BasicClient::new(
            discord_client_id,
            Some(discord_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_url(
            RedirectUrl::new(format!("{}/api/v1/auth/callback", cfg.http.url))
                .expect("Invalid redirect URL"),
        );

        App::new()
            .data(AppState {
                oauth: oauth_client,
            })
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .configure(routes::config)
    })
    .bind((cfg.http.bind, cfg.http.port))?
    .run()
    .await
}
