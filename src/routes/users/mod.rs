mod hanlder;
use actix_web::web;
pub use hanlder::{create_user, get_users};

pub fn config_users(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users).service(create_user);
}
