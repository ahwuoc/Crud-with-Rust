use actix_web::{Responder, get};

#[get("/orders")]
pub async fn get_orders() -> impl Responder {
    "Danh sách orders"
}
