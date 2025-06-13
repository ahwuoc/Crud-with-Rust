use actix_web::{HttpResponse, Responder, get, web};
use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize, sqlx::FromRow)]
struct Order {
    id: i64,
    user_id: i64,
    order_status: String,
    total_price: BigDecimal,
}

#[get("/orders")]
pub async fn get_orders(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Order,
        "SELECT id, user_id, order_status, total_price FROM orders"
    )
    .fetch_all(&**pool)
    .await;

    match result {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("Lỗi khi truy vấn orders: {}", e);
            actix_web::HttpResponse::InternalServerError().body("Lỗi khi lấy danh sách orders")
        }
    }
}
