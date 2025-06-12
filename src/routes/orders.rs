use actix_web::{HttpResponse, Responder, get};

#[get("/api/orders")]
pub async fn get_orders() -> impl Responder {
    HttpResponse::Ok().body("API đang sống nè!")
}
