use super::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ticket")
    );
}
