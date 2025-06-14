mod config;
mod routes;
use crate::config::get_database_pool;
use crate::routes::orders::config_orders;
use crate::routes::products::config_products;
use crate::routes::users::config_users;
use actix_web::{App, HttpServer, web};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // this is function get pool for database
    let pool = get_database_pool().await;
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 3000;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config_users)
            .configure(config_products)
            .configure(config_orders)
    })
    .bind((HOST, PORT));
    match server {
        Ok(server) => {
            println!("🚀 Server running at http://{}:{}", HOST, PORT);
            server.run().await
        }
        Err(e) => {
            eprintln!(
                "❌ Failed to bind server to http://{}:{}: {}",
                HOST, PORT, e
            );
            Err(e)
        }
    }
}
