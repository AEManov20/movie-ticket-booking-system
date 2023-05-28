use super::*;

use crate::{
    model::{FormMovie, JwtClaims, JwtType, User},
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

#[derive(Serialize)]
#[serde(tag = "type")]
enum QueryErrorType {
    ValidationErrors(ValidationErrors),
}

#[post("/review/new")]
async fn submit_new_review(
    new_review: web::Json<NewReviewPayload>,
    user_service: web::Data<UserService>,
    claims: JwtClaims,
) -> HttpResponse {
    match user_res_from_jwt(&claims, &user_service).await {
        Ok(user_res) => {
            let result = user_res.create_review(new_review.content.clone(), new_review.rating, new_review.movie_id).await;

            match result {
                Ok(review) => HttpResponse::Ok().json(review),
                Err(e) => {
                    error!("{:?}", e);
                    HttpResponse::InternalServerError().into()
                }
            }
        },
        Err(e) => HttpResponse::InternalServerError().json(e)
    }
}

#[get("/review/{id}")]
async fn get_review_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> HttpResponse {
    match movie_service.get_review_by_id(path.0).await {
        Ok(v) => {
            match v {
                Some(v) => HttpResponse::Ok().json(v),
                None => HttpResponse::NotFound().into()
            }
        },
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().into()
        }
    }
}

#[delete("/review/{id}")]
async fn delete_review_by_id(path: web::Path<(uuid::Uuid,)>, user_service: web::Data<UserService>, movie_service: web::Data<MovieService>, claims: JwtClaims) -> HttpResponse {
    let review = movie_service.get_review_by_id(path.0).await;
    let user = user_res_from_jwt(&claims, &user_service).await;

    match user {
        Ok(user) => {
            let user = User::from(user);

            match review {
                Ok(v) => {
                    let Some(v) = v else {
                        return HttpResponse::NotFound().into()
                    };
        
                    if v.author_user_id == user.id {
                        if let Err(e) = movie_service.delete_review_by_id(v.id).await {
                            error!("{:?}", e);
                            HttpResponse::InternalServerError().into()
                        } else {
                            HttpResponse::Ok().into()
                        }
                    } else if user.is_super_user {
                        if let Err(e) = movie_service.delete_review_by_id(v.id).await {
                            error!("{:?}", e);
                            HttpResponse::InternalServerError().into()
                        } else {
                            HttpResponse::Ok().into()
                        }
                    } else {
                        HttpResponse::Forbidden().into()
                    }
                },
                Err(e) => {
                    error!("{:?}", e);
                    HttpResponse::InternalServerError().into()
                }
            }
        },
        Err(e) => HttpResponse::Forbidden().json(e)
    }
}

#[get("/{id}/reviews")]
async fn get_reviews(path: web::Path<(uuid::Uuid,)>, query: web::Query<MovieReviewQuery>, movie_service: web::Data<MovieService>) -> HttpResponse {
    match query.validate() {
        Ok(_) => {
            match movie_service.query_reviews(query.movie_id, query.limit, query.offset, query.sort_by).await {
                Ok(v) => HttpResponse::Ok().json(v),
                Err(e) => {
                    error!("{:?}", e);
                    HttpResponse::InternalServerError().into()
                }
            }
        },
        Err(e) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: QueryErrorType::ValidationErrors(e) })
        }
    }
}

#[get("/{id}/theatres")]
async fn get_theatres_by_movie_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> HttpResponse {
    todo!();
}

#[get("/query")]
async fn query_movies(
    query: web::Query<MovieQuery>,
    movie_service: web::Data<MovieService>,
) -> HttpResponse {
    if let Err(e) = query.validate() {
        HttpResponse::BadRequest().json(ErrorResponse {
            error: QueryErrorType::ValidationErrors(e),
        })
    } else {
        let result = movie_service
            .query_movies(query.name.clone(), query.limit, query.offset, query.sort_by)
            .await
            .or_else(|x| {
                error!("{}", x);
                Err(x)
            });

        let Ok(result) = result else {
            return HttpResponse::InternalServerError().into();
        };

        HttpResponse::Ok().json(result)
    }
}

#[get("/{id}")]
async fn get_movie_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>) -> HttpResponse {
    match movie_service.get_by_id(path.0).await {
        Ok(movie) => {
            let Some(movie) = movie else {
                return HttpResponse::NotFound().into()
            };
            
            HttpResponse::Ok().json(movie)
        },
        Err(e) => {
            error!("{:?}", e);

            HttpResponse::InternalServerError().into()
        }
    }
}

#[delete("/{id}")]
async fn delete_movie_by_id(path: web::Path<(uuid::Uuid,)>, movie_service: web::Data<MovieService>, user_service: web::Data<UserService>, claims: JwtClaims) -> HttpResponse {
    match user_res_from_jwt(&claims, &user_service).await {
        Ok(user) => {
            let user = User::from(user);
            if user.is_super_user {
                match movie_service.delete(path.0).await {
                    Ok(_) => HttpResponse::Ok().into(),
                    Err(e) => {
                        error!("{:?}", e);
                        HttpResponse::InternalServerError().into()
                    }
                }
            } else {
                HttpResponse::Forbidden().into()
            }
        },
        Err(e) => HttpResponse::Forbidden().json(e)
    }
}

#[post("/new")]
async fn insert_movie(movie: web::Json<FormMovie>, movie_service: web::Data<MovieService>, user_service: web::Data<UserService>, claims: JwtClaims) -> HttpResponse {
    if let Err(e) = movie.validate() {
        HttpResponse::BadRequest().json(e)
    } else {
        match user_res_from_jwt(&claims, &user_service).await {
            Ok(user) => {
                let user = User::from(user);
                if user.is_super_user {
                    match movie_service.create(movie.into_inner()).await {
                        Ok(v) => HttpResponse::Ok().json(v),
                        Err(e) => {
                            error!("{:?}", e);
                            HttpResponse::InternalServerError().into()
                        }
                    }
                } else {
                    HttpResponse::Forbidden().into()
                }
            },
            Err(e) => {
                error!("{:?}", e);
                HttpResponse::InternalServerError().into()
            }
        }
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
