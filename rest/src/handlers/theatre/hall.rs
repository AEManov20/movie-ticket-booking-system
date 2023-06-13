// TODO!

use crate::model::Hall;

use super::*;

// #[get("/all")]
// async fn get_halls(path: web::Path<uuid::Uuid>, theatre_service: web::Data<TheatreService>) -> Result<Vec<Hall>> {
//     let theatre_id = path.into_inner();
//     Ok(theatre_service.get_by_id(id_))
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hall")
            // .service(get_halls)
            // .service(create_hall)
            // .service(delete_hall)
    );
}
