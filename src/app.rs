use crate::routes::orders::get_orders;
use actix_web::dev::HttpServiceFactory;
use actix_web::{App, web};
use sqlx::MySqlPool;

pub fn create_app(pool: MySqlPool) -> impl actix_web::dev::HttpServiceFactory {
    App::new()
        .app_data(web::Data::new(pool))
        .service(get_orders)
}
