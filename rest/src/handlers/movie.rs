use super::*;

use crate::{
    model::{FormMovie, JwtClaims, JwtType, MovieReview, User, Movie, Theatre},
    services::{
        SortBy,
        movie::MovieService,
        user::UserService,
    },
};

#[derive(Deserialize)]
struct NewReviewPayload {
    movie_id: uuid::Uuid,
    content: Option<String>,
    rating: f64,
}

#[derive(Deserialize, Validate)]
struct MovieQuery {
    #[validate(length(min = 1, max = 250))]
    name: Option<String>,
    sort_by: SortBy,
    #[validate(range(min = 1, max = 100))]
    limit: i64,
    offset: i64,
}

#[derive(Deserialize, Validate)]
struct MovieReviewQuery {
    movie_id: uuid::Uuid,
    #[validate(range(min = 1, max = 100))]
    limit: i64,
    offset: i64,
    sort_by: SortBy
}

#[post("/review/new")]
async fn submit_new_review(
    new_review: web::Json<NewReviewPayload>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> Result<MovieReview> {
    let (user_res, _) = user_res_from_jwt(&claims, &user_service).await?;

    match user_res.create_review(new_review.content.clone(), new_review.rating, new_review.movie_id).await? {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::ServerError)
    }
}

#[get("/review/{id}")]
async fn get_review_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> Result<MovieReview> {
    match movie_service.get_review_by_id(path.0).await? {
        Some(v) => Ok(v.into()),
        None => Err(ErrorType::NotFound)
    }
}

#[delete("/review/{id}")]
async fn delete_review_by_id(path: web::Path<(uuid::Uuid,)>, user_service: web::Data<UserService>, movie_service: web::Data<MovieService>, claims: JwtClaims) -> Result<()> {
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

#[get("/{id}/reviews")]
async fn get_reviews(path: web::Path<(uuid::Uuid,)>, query: web::Query<MovieReviewQuery>, movie_service: web::Data<MovieService>) -> Result<Vec<MovieReview>> {
    query.validate()?;

    Ok(movie_service.query_reviews(query.movie_id, query.limit, query.offset, query.sort_by).await?.into())
}

#[get("/{id}/theatres")]
async fn get_theatres_by_movie_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> Result<Vec<Theatre>> {
    todo!();
}

#[get("/query")]
async fn query_movies(
    query: web::Query<MovieQuery>,
    movie_service: web::Data<MovieService>,
) -> Result<Vec<Movie>> {
    query.validate()?;

    Ok(movie_service.query_movies(query.name.clone(), query.limit, query.offset, query.sort_by).await?.into())
}

#[get("/{id}")]
async fn get_movie_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> Result<Movie> {
    match movie_service.get_by_id(path.0).await? {
        Some(v) => {
            if !v.is_deleted { Ok(v.into()) }
            else { Err(ErrorType::NotFound) }
        },
        None => Err(ErrorType::NotFound)
    }
}

#[delete("/{id}")]
async fn delete_movie_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>, user_service: web::Data<UserService>, claims: JwtClaims) -> Result<()> {
    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;

    if user.is_super_user {
        Ok(movie_service.delete(path.0).await?.into())
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

#[post("/new")]
async fn insert_movie(movie: web::Json<FormMovie>, movie_service: web::Data<MovieService>, user_service: web::Data<UserService>, claims: JwtClaims) -> Result<Movie> {
    movie.validate()?;

    let (_, user) = user_res_from_jwt(&claims, &user_service).await?;
    if user.is_super_user {
        match movie_service.create(movie.into_inner()).await? {
            Some(v) => Ok(v.into()),
            None => Err(ErrorType::ServerError)
        }
    } else {
        Err(ErrorType::InsufficientPermission)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/movie")
            .service(submit_new_review)
            .service(get_review_by_id)
            .service(delete_review_by_id)
            .service(get_theatres_by_movie_id)
            .service(query_movies)
            .service(get_movie_by_id)
            .service(delete_movie_by_id)
            .service(insert_movie),
    );
}
