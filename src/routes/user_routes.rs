use actix_web::{get, post, web, Error, HttpResponse, Responder};

use diesel::prelude::*;

use crate::models::user::{Login, LoginSuccess, NewUser, User};
use crate::utils::hash;
use crate::utils::jwtauth::create_token;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/register")]
pub async fn register(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> Result<User, Error> {
    NewUser::create_user(pool, user.into_inner()).await
}

#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, user: web::Json<Login>) -> impl Responder {
    let user = user.into_inner();
    let password = user.password.clone();
    Login::login(pool, user).await.map(|user| {
        if hash::verify_password(&user.password, &password) {
            let token = create_token(user.username, user.email);
            HttpResponse::Ok().json(LoginSuccess {
                status: "success".to_string(),
                token: token.to_string(),
            })
        } else {
            let json = serde_json::to_string(&LoginSuccess {
                status: "fail".to_string(),
                token: "".to_string(),
            });
            HttpResponse::Forbidden()
                .content_type("application/json")
                .body(json.unwrap())
        }
    })
}

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let users = User::get_users(pool).await;
    HttpResponse::Ok().json(users)
}

#[get("/users/username/{username}")]
async fn get_user_by_name(username: web::Path<String>, pool: web::Data<DbPool>) -> impl Responder {
    let user = User::get_user_by_username(username.to_string(), pool).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/users/id/{id}")]
async fn get_user_by_id(id: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    // let id = id.parse::<i64>().unwrap();
    let user = User::get_user_by_id(*id, pool).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
