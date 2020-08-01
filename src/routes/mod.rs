mod v1;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").configure(v1::config));
}
