use std::sync::Arc;

use chrono::{Datelike, Utc};
use validator::ValidationError;

use crate::model::{FormTheatreScreening, TheatreScreening, TheatreScreeningEvent};

use super::*;

#[derive(Deserialize)]
pub struct TimelineQuery {
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: Option<chrono::DateTime<Utc>>,
}

// TODO: implement verbose validation error messages
fn validate_timeline_query(query: &TimelineQuery) -> std::result::Result<(), ValidationErrors> {
    let now = Utc::now();

    if now.date_naive() > query.start_date.date_naive() {
        return Err(ValidationErrors::new());
    }

    if let Some(end_date) = query.end_date {
        if end_date > query.start_date {
            return Err(ValidationErrors::new());
        }
    }

    Ok(())
}

/// Fetches screening events over a given timespan
#[utoipa::path(context_path = "/api/v1/theatre/{id}/screening")]
#[get("/timeline")]
pub async fn get_timeline(
    path: web::Path<(uuid::Uuid,)>,
    query: web::Path<TimelineQuery>,
    theatre_service: web::Data<TheatreService>,
) -> Result<Vec<TheatreScreeningEvent>> {
    validate_timeline_query(query.as_ref())?;

    let theatre_id = path.0;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    Ok(theatre_res
        .query_screening_events(
            query.start_date.naive_utc(),
            query.end_date.map(|x| x.naive_utc()),
        )
        .await?
        .into())
}

/// Fetches data about a given theatre screening by ID
#[utoipa::path(context_path = "/api/v1/theatre/{id}/screening")]
#[get("/{tsid}")]
pub async fn get_theatre_screening(
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    theatre_service: web::Data<TheatreService>,
) -> Result<TheatreScreening> {
    let (theatre_id, theatre_screening_id) = path.into_inner();
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    match theatre_res
        .get_theatre_screening(theatre_screening_id)
        .await?
    {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound),
    }
}

/// Updates a theatre screening
#[utoipa::path(context_path = "/api/v1/theatre/{id}/screening")]
#[put("/{tsid}")]
pub async fn update_theatre_screening(
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    new_theatre_screening: web::Json<FormTheatreScreening>,
    user_service: web::Data<UserService>,
    theatre_service: web::Data<TheatreService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    role_service: web::Data<RoleService>,
    claims: JwtClaims,
) -> Result<TheatreScreening> {
    let (theatre_id, theatre_screening_id) = path.into_inner();
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles!(
            [Role::ScreeningsManager],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(theatre_res
        .update_theatre_screening(theatre_screening_id, new_theatre_screening.into_inner())
        .await?
        .into())
}

/// Deletes a theatre screening
#[utoipa::path(context_path = "/api/v1/theatre/{id}/screening")]
#[delete("/{tsid}")]
pub async fn delete_theatre_screening(
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    user_service: web::Data<UserService>,
    theatre_service: web::Data<TheatreService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    role_service: web::Data<RoleService>,
    claims: JwtClaims,
) -> Result<()> {
    let (theatre_id, theatre_screening_id) = path.into_inner();
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles!(
            [Role::TheatreOwner, Role::ScreeningsManager],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(theatre_res
        .delete_theatre_screening(theatre_screening_id)
        .await?
        .into())
}

/// Creates a new theatre screening
#[utoipa::path(context_path = "/api/v1/theatre/{id}/screening")]
#[post("/new")]
pub async fn create_theatre_screening(
    path: web::Path<uuid::Uuid>,
    new_theatre_screening: web::Json<FormTheatreScreening>,
    user_service: web::Data<UserService>,
    theatre_service: web::Data<TheatreService>,
    bridge_role_service: web::Data<BridgeRoleService>,
    role_service: web::Data<RoleService>,
    claims: JwtClaims,
) -> Result<TheatreScreening> {
    let theatre_id = path.into_inner();
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(theatre_res) = theatre_service.get_by_id(theatre_id).await? else {
        return Err(ErrorType::NotFound)
    };

    if !user.is_super_user {
        check_roles!(
            [Role::TheatreOwner, Role::ScreeningsManager],
            user.id,
            theatre_id,
            bridge_role_service,
            role_service
        );
    }

    Ok(theatre_res
        .create_theatre_screening(new_theatre_screening.into_inner())
        .await?
        .into())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/screening")
            .service(get_timeline)
            .service(update_theatre_screening)
            .service(delete_theatre_screening)
            .service(create_theatre_screening)
            .service(get_theatre_screening),
    );
}
