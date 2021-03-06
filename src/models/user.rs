use std::future::{ready, Ready};

use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::users;
use crate::utils::hash;
use diesel::r2d2::{self, ConnectionManager};
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    #[serde(skip)]
    pub uid: Uuid,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing)]
    #[validate(length(min = 8, max = 50))]
    pub password: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Queryable)]
pub struct User {
    pub id: i64,
    pub uid: Uuid,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing)]
    #[validate(length(min = 8, max = 50))]
    pub password: String,
    #[serde(skip)]
    pub sign_in_count: i32,
    #[serde(skip)]
    current_sign_in_at: Option<NaiveDateTime>,
    #[serde(skip)]
    last_sign_in_at: Option<NaiveDateTime>,
    #[serde(skip)]
    deleted_at: Option<NaiveDateTime>,
    #[serde(skip)]
    created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Login {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 3, max = 50))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSuccess {
    pub status: String,
    pub token: String,
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
impl User {
    pub async fn get_user_by_id(id: i64, pool: web::Data<DbPool>) -> Result<User, Error> {
        let conn = pool.get().unwrap();
        let user =
            web::block(move || users::table.filter(users::id.eq(id)).first::<User>(&conn)).await?;

        Ok(user)
    }

    pub async fn get_user_by_username(
        username: String,
        pool: web::Data<DbPool>,
    ) -> Result<User, Error> {
        let conn = pool.get().unwrap();
        let user = web::block(move || {
            users::table
                .filter(users::username.eq(username))
                .first::<User>(&conn)
        })
        .await?;

        Ok(user)
    }

    pub async fn get_users(pool: web::Data<DbPool>) -> Vec<User> {
        web::block(move || {
            let conn = pool.get().unwrap();
            users::table.load::<User>(&conn)
        })
        .await
        .unwrap()
    }
}

impl NewUser {
    pub async fn create_user(pool: web::Data<DbPool>, user: NewUser) -> Result<User, Error> {
        let mut user = user;
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
}

impl Login {
    pub async fn login(pool: web::Data<DbPool>, user: Login) -> Result<User, Error> {
        let conn = pool.get().unwrap();
        let user = web::block(move || {
            users::table
                .filter(users::username.eq(&user.username))
                .first::<User>(&conn)
        })
        .await?;
        Ok(user)
    }
}
