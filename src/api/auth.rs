use actix_web::{get, web, HttpResponse, Responder};

#[get("/test")]
pub async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("test")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(get_test));
}
