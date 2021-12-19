use actix_web::{post, web, Error, HttpResponse, Responder};

use diesel::prelude::*;

use crate::models::user::{Login, LoginSuccess, NewUser, User};
use crate::utils::hash;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use crate::schema::users;

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
