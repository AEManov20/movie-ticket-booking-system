use utoipa::{IntoParams, ToSchema};

use super::*;

use crate::{
    doc,
    model::{
        ExtendedUserReview, FormMovie, FormMovieReview, JwtClaims, Movie, MovieReview, Theatre,
        UpdateMovieReview,
    },
    services::{movie::MovieService, user::UserService, SortBy},
};

#[derive(Deserialize, Validate, ToSchema, IntoParams)]
pub struct MovieQuery {
    #[validate(length(min = 1, max = 250))]
    pub name: Option<String>,
    pub sort_by: SortBy,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
    pub offset: i64,
}

#[derive(Deserialize, Validate, ToSchema, IntoParams)]
pub struct MovieReviewQuery {
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
    pub offset: i64,
    pub sort_by: SortBy,
}

/// Creates a new review for a given movie
#[utoipa::path(
    context_path = "/api/v1/movie",
    request_body = FormMovieReview,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "Review created successfully and returned", body = MovieReview),
    ),
    security(
        ("api_key" = [])
    )
)]
#[post("/review/new")]
pub async fn submit_new_review(
    new_review: web::Json<FormMovieReview>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<MovieReview> {
    new_review.validate()?;

    let (user_res, _) = user_res_from_jwt(&claims, &user_service).await?;

    Ok(user_res
        .create_review(new_review.into_inner())
        .await?
        .into())
}

#[derive(Deserialize, IntoParams)]
pub struct UpdateReviewQuery {
    pub review_id: uuid::Uuid,
}

/// Updates a review by ID given that the user has ownership/permission
#[utoipa::path(
    context_path = "/api/v1/movie",
    request_body = UpdateMovieReview,
    params(
        UpdateReviewQuery
    ),
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions"),
        (status = NOT_FOUND, description = "The selected review was not found"),
        (status = OK, description = "Review updated and returned successfully", body = MovieReview)
    ),
    security(
        ("api_key" = [])
    )
)]
#[put("/review/{id}")]
pub async fn update_review_by_id(
    path: web::Path<(uuid::Uuid,)>,
    query: web::Query<UpdateReviewQuery>,
    new_review: web::Json<UpdateMovieReview>,
    movie_service: web::Data<MovieService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<MovieReview> {
    let (user_res, user) = user_res_from_jwt(&claims, &user_service).await?;
    let Some(review) = movie_service.get_review_by_id(path.0).await? else {
        return Err(ErrorType::NotFound)
    };

    if review.author_user_id == user.id {
        Ok(user_res
            .update_review(
                query.review_id,
                new_review.content.clone(),
                new_review.rating,
            )
            .await?
            .into())
    } else if user.is_super_user {
        Ok(user_res
            .update_review(
                query.review_id,
                new_review.content.clone(),
                new_review.rating,
            )
            .await?
            .into())
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

/// Gets a review by ID
#[utoipa::path(
    context_path = "/api/v1/movie",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = NOT_FOUND, description = "The selected review was not found"),
        (status = OK, description = "MovieReview found and returned", body = MovieReview),
    )
)]
#[get("/review/{id}")]
pub async fn get_review_by_id(
    path: web::Path<(uuid::Uuid,)>,
    movie_service: web::Data<MovieService>,
) -> Result<MovieReview> {
    match movie_service.get_review_by_id(path.0).await? {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound),
    }
}

/// Deletes a review by ID given that the user has ownership/permission
#[utoipa::path(
    context_path = "/api/v1/movie",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions"),
        (status = NOT_FOUND, description = "The selected review was not found"),
        (status = OK, description = "Review deleted successfully")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/review/{id}")]
pub async fn delete_review_by_id(
    path: web::Path<(uuid::Uuid,)>,
    user_service: web::Data<UserService>,
    movie_service: web::Data<MovieService>,
    claims: JwtClaims,
) -> Result<()> {
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
#[utoipa::path(
    context_path = "/api/v1/movie",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "Review query completed successfully and returned", body = Vec<MovieReview>)
    ),
    params(
        MovieReviewQuery
    )
)]
#[get("/{id}/reviews")]
pub async fn get_reviews(
    path: web::Path<(uuid::Uuid,)>,
    query: web::Query<MovieReviewQuery>,
    movie_service: web::Data<MovieService>,
) -> Result<Vec<ExtendedUserReview>> {
    query.validate()?;

    Ok(movie_service
        .query_reviews(path.0, query.limit, query.offset, query.sort_by)
        .await?
        .into())
}

/// Queries movies given certain criteria
#[utoipa::path(
    context_path = "/api/v1/movie",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "Movie query completed successfully and returned results", body = Vec<Movie>)
    ),
    params(
        MovieQuery
    )
)]
#[get("/query")]
pub async fn query_movies(
    query: web::Query<MovieQuery>,
    movie_service: web::Data<MovieService>,
) -> Result<Vec<Movie>> {
    query.validate()?;

    Ok(movie_service
        .query_movies(query.name.clone(), query.limit, query.offset, query.sort_by)
        .await?
        .into())
}

/// Gets a movie by ID
#[utoipa::path(
    context_path = "/api/v1/movie",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = NOT_FOUND, description = "The selected movie was not found"),
        (status = OK, description = "Movie found and returned", body = Movie)
    )
)]
#[get("/{id}")]
pub async fn get_movie_by_id(
    path: web::Path<(uuid::Uuid,)>,
    movie_service: web::Data<MovieService>,
) -> Result<Movie> {
    match movie_service.get_by_id(path.0).await? {
        Some(v) => {
            if !v.is_deleted {
                Ok(v.into())
            } else {
                Err(ErrorType::NotFound)
            }
        }
        None => Err(ErrorType::NotFound),
    }
}

/// Deletes a movie by ID (superuser only)
#[utoipa::path(
    context_path = "/api/v1/movie",
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions"),
        (status = NOT_FOUND, description = "The selected movie was not found"),
        (status = OK, description = "Movie found and deleted successfully")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/{id}")]
pub async fn delete_movie_by_id(
    path: web::Path<(uuid::Uuid,)>,
    movie_service: web::Data<MovieService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<()> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if user.is_super_user {
        Ok(movie_service.delete(path.0).await?.into())
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

/// Creates a new movie (superuser only)
#[utoipa::path(
    context_path = "/api/v1/movie",
    request_body = FormMovie,
    responses(
        (status = "5XX", description = "Internal server error has occurred (database/misc)"),
        (status = UNAUTHORIZED, description = "User hasn't authenticated yet"),
        (status = FORBIDDEN, description = "User doesn't meet the required permissions"),
        (status = BAD_REQUEST, description = "Invalid data supplied"),
        (status = OK, description = "Movie created successfully and returned", body = Movie)
    ),
    security(
        ("api_key" = [])
    )
)]
#[post("/new")]
pub async fn create_movie(
    movie: web::Json<FormMovie>,
    movie_service: web::Data<MovieService>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<Movie> {
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
            .service(create_movie)
            .service(query_movies)
            .service(submit_new_review)
            .service(get_review_by_id)
            .service(delete_review_by_id)
            .service(get_movie_by_id)
            .service(delete_movie_by_id)
            .service(get_reviews),
    );
}
