use std::collections::HashMap;

use crate::{model::Language, services::language::LanguageService};

use super::*;

/// Gets all possible languages as a dictionary
#[utoipa::path(
    context_path = "/api/v1/language",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = OK, description = "Database operations were successful and the languages have been returned", body = HashMap<String, uuid::Uuid>)
    )
)]
#[get("/all")]
pub async fn get_all_languages(
    language_service: web::Data<LanguageService>,
) -> HandlerResult<HashMap<String, uuid::Uuid>> {
    Ok(language_service
        .get_all_languages()
        .await?
        .iter()
        .map(|x| (x.code.clone(), x.id))
        .collect::<HashMap<_, _>>()
        .into())
}

/// Gets a certain language by ID
#[utoipa::path(
    context_path = "/api/v1/language",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = NOT_FOUND, description = "The language was not found"),
        (status = OK, description = "The language has been found and returned", body = Language)
    )
)]
#[get("/{id}")]
pub async fn get_language(
    path: web::Path<uuid::Uuid>,
    language_service: web::Data<LanguageService>,
) -> HandlerResult<Language> {
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
            .service(get_all_languages)
            .service(get_language),
    );
}
