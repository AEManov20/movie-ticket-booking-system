use utoipa::ToSchema;

use super::*;

use crate::{
    model::{FormMovie, JwtClaims, MovieReview, Movie, Theatre},
    services::{
        SortBy,
        movie::MovieService,
        user::UserService,
    },
};

#[derive(Deserialize, ToSchema)]
pub struct NewReviewPayload {
    pub movie_id: uuid::Uuid,
    #[schema(example = "This movie is the best 11/10.")]
    pub content: Option<String>,
    #[schema(example = 0.95)]
    pub rating: f64,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct MovieQuery {
    #[validate(length(min = 1, max = 250))]
    pub name: Option<String>,
    pub sort_by: SortBy,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
    pub offset: i64,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct MovieReviewQuery {
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
    pub offset: i64,
    pub sort_by: SortBy
}

/// Creates a new review for a given movie
#[utoipa::path(context_path = "/api/v1/movie")]
#[post("/review/new")]
pub async fn submit_new_review(
    new_review: web::Json<NewReviewPayload>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<MovieReview> {
    let (user_res, _) = user_res_from_jwt(&claims, &user_service).await?;

    Ok(user_res.create_review(new_review.content.clone(), new_review.rating, new_review.movie_id).await?.into())
}

/// Gets a review by ID
#[utoipa::path(context_path = "/api/v1/movie")]
#[get("/review/{id}")]
pub async fn get_review_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> Result<MovieReview> {
    match movie_service.get_review_by_id(path.0).await? {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound)
    }
}

/// Deletes a review by ID given that the user has ownership/permission
#[utoipa::path(context_path = "/api/v1/movie")]
#[delete("/review/{id}")]
pub async fn delete_review_by_id(path: web::Path<(uuid::Uuid,)>, user_service: web::Data<UserService>, movie_service: web::Data<MovieService>, claims: JwtClaims) -> Result<()> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(review) = movie_service.get_review_by_id(path.0).await? else {
        return Err(ErrorType::NotFound)
    };

    if review.author_user_id == user.id {
        Ok(movie_service.delete_review_by_id(review.id).await?.into())
    } else if user.is_super_user {
        Ok(movie_service.delete_review_by_id(review.id).await?.into())
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

/// Queries reviews for a given movie
#[utoipa::path(context_path = "/api/v1/movie")]
#[get("/{id}/reviews")]
pub async fn get_reviews(path: web::Path<(uuid::Uuid,)>, query: web::Query<MovieReviewQuery>, movie_service: web::Data<MovieService>) -> Result<Vec<MovieReview>> {
    query.validate()?;

    Ok(movie_service.query_reviews(path.0, query.limit, query.offset, query.sort_by).await?.into())
}

/// Gets theatres where movie is screened
#[utoipa::path(context_path = "/api/v1/movie")]
#[get("/{id}/theatres")]
pub async fn get_theatres_by_movie_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> Result<Vec<Theatre>> {
    todo!();
}

/// Queries movies given certain criteria
#[utoipa::path(context_path = "/api/v1/movie")]
#[get("/query")]
pub async fn query_movies(
    query: web::Query<MovieQuery>,
    movie_service: web::Data<MovieService>,
) -> Result<Vec<Movie>> {
    query.validate()?;

    Ok(movie_service.query_movies(query.name.clone(), query.limit, query.offset, query.sort_by).await?.into())
}

/// Gets a movie by ID
#[utoipa::path(context_path = "/api/v1/movie")]
#[get("/{id}")]
pub async fn get_movie_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> Result<Movie> {
    match movie_service.get_by_id(path.0).await? {
        Some(v) => {
            if !v.is_deleted { Ok(v.into()) }
            else { Err(ErrorType::NotFound) }
        },
        None => Err(ErrorType::NotFound)
    }
}

/// Deletes a movie by ID (superuser only)
#[utoipa::path(context_path = "/api/v1/movie")]
#[delete("/{id}")]
pub async fn delete_movie_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>, user_service: web::Data<UserService>, claims: JwtClaims) -> Result<()> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if user.is_super_user {
        Ok(movie_service.delete(path.0).await?.into())
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

/// Creates a new movie (superuser only)
#[utoipa::path(context_path = "/api/v1/movie")]
#[post("/new")]
pub async fn create_movie(movie: web::Json<FormMovie>, movie_service: web::Data<MovieService>, user_service: web::Data<UserService>, claims: JwtClaims) -> Result<Movie> {
    movie.validate()?;

    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    if user.is_super_user {
        Ok(movie_service.create(movie.into_inner()).await?.into())
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/movie")
            .service(submit_new_review)
            .service(query_movies)
            .service(create_movie)
            .service(get_review_by_id)
            .service(delete_review_by_id)
            .service(get_theatres_by_movie_id)
            .service(get_movie_by_id)
            .service(delete_movie_by_id),
    );
}
