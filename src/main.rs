mod app;
mod config;
mod routes;
use crate::app::create_app;
use crate::config::get_database_pool;
use actix_web::HttpServer;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_database_pool().await;
    // HttpServer::new(move || create_app(pool.clone()))
    //     .bind("127.0.0.1:3000")?
    //     .run()
    //     .await?;
    // Ok(())
}
