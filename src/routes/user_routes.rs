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
