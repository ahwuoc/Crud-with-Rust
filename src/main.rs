mod config;
mod routes;
use crate::config::get_database_pool;
use crate::routes::orders::get_orders;
use crate::routes::users::{create_user, get_users};
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_database_pool().await;
    let host = "127.0.0.1";
    let port = 3000;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_orders)
            .service(get_users)
            .service(create_user)
    })
    .bind((host, port));
    match server {
        Ok(server) => {
            println!("🚀 Server running at http://{}:{}", host, port);
            server.run().await
        }
        Err(e) => {
            eprintln!(
                "❌ Failed to bind server to http://{}:{}: {}",
                host, port, e
            );
            Err(e)
        }
    }
}
