use actix_web::{HttpResponse, Responder, get, post, web};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Product {
    id: i64,
    name: String,
    description: Option<String>,
    price: BigDecimal,
    stock: Option<i32>,
    category_id: Option<i64>,
    category_name: Option<String>,
}
#[get("/products")]
pub async fn get_products(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Product,
        "SELECT p.id , p.name , p.description , p.price , p.stock , c.id as category_id , c.name as category_name   FROM products p LEFT JOIN categories c ON p.category_id = c.id",
    )
    .fetch_all(&**pool)
    .await;
    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
#[post("/products")]
pub async fn create_product(
    pool: web::Data<MySqlPool>,
    product: web::Json<Product>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO products (name, description, price, stock) VALUES (?, ?, ?, ?)",
        product.name,
        product.description,
        product.price,
        product.stock
    )
    .execute(&**pool)
    .await;
    match result {
        Ok(_) => HttpResponse::Created().json(product),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
