use super::*;

// TODO!

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
    );
}