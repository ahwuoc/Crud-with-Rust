pub mod handler;
use actix_web::web;
use handler::{create_product, get_products};

pub fn config_products(cfg: &mut web::ServiceConfig) {
    cfg.service(create_product).service(get_products);
}
