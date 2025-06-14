mod handler;
use actix_web::web;
use handler::get_orders;

pub fn config_orders(cfg: &mut web::ServiceConfig) {
    cfg.service(get_orders);
}
