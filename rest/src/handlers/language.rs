use crate::{model::Language, services::language::LanguageService};

use super::*;

#[utoipa::path(context_path = "/api/v1/language")]
#[get("/all")]
pub async fn get_all_languages(language_service: web::Data<LanguageService>) -> Result<Vec<Language>> {
    Ok(language_service.get_all_languages().await?.into())
}

#[utoipa::path(context_path = "/api/v1/language")]
#[get("/{id}")]
pub async fn get_language(
    path: web::Path<uuid::Uuid>,
    language_service: web::Data<LanguageService>,
) -> Result<Language> {
    match language_service
        .get_language_by_id(path.into_inner())
        .await?
    {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/language")
            .service(get_language)
            .service(get_all_languages),
    );
}
