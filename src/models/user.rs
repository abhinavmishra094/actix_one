use std::future::{ready, Ready};

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;

use diesel::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    #[serde(skip)]
    pub uid: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    #[serde(skip)]
    pub id: i64,
    #[serde(skip)]
    pub uid: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
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
