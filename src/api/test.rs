use actix_web::{get, HttpResponse, Responder};

#[get("/test")]
pub async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("test")
}
