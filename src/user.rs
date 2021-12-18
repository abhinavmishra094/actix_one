use std::future::{ready, Ready};

use actix_web::{http, post, web, Error, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::prelude::*;

use diesel::{
    r2d2::{self, ConnectionManager},
    Insertable, PgConnection,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::hash;
use crate::schema::users;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    #[serde(skip)]
    uid: Uuid,
    username: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    #[serde(skip)]
    id: i64,
    #[serde(skip)]
    uid: Uuid,
    username: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
    #[serde(skip)]
    sign_in_count: i32,
    #[serde(skip)]
    current_sign_in_at: Option<NaiveDateTime>,
    #[serde(skip)]
    last_sign_in_at: Option<NaiveDateTime>,
    #[serde(skip)]
    deleted_at: Option<NaiveDateTime>,
    #[serde(skip)]
    created_at: Option<NaiveDateTime>,
}
fn now() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSuccess {
    status: String,
    token: String,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
#[post("/register")]
pub async fn register(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> Result<User, Error> {
    println!("{:?}", user);
    let mut user = user.into_inner();
    println!("{:?}", user);
    user.uid = Uuid::new_v4();
    user.password = hash::hash_password(&user.password);
    let user = web::block(move || {
        let conn = pool.get().unwrap();
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&conn)
    })
    .await?;

    Ok(user)
}

#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, user: web::Json<Login>) -> impl Responder {
    let user = user.into_inner();
    let password = user.password.clone();
    let conn = pool.get().unwrap();
    let user = web::block(move || {
        users::table
            .filter(users::username.eq(&user.username))
            .first::<User>(&conn)
    })
    .await
    .expect("Error getting user");

    if hash::verify_password(&user.password, &password) {
        let json = serde_json::to_string(&LoginSuccess {
            status: "success".to_string(),
            token: "".to_string(),
        });
        HttpResponse::Ok()
            .content_type("application/json")
            .body(json.unwrap())
    } else {
        let json = serde_json::to_string(&LoginSuccess {
            status: "fail".to_string(),
            token: "".to_string(),
        });
        HttpResponse::Forbidden()
            .content_type("application/json")
            .body(json.unwrap())
    }
}
