mod middelware;
mod models;
mod routes;
mod schema;
mod utils;
extern crate dotenv;
extern crate uuid;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;
use std::env;

use actix_web::{middleware::Logger, App, HttpServer};

use dotenv::dotenv;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection = utils::database::create_database_pool();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .data(connection.clone())
            .wrap(Logger::new(
                "\nRemote IP Address %a\nTime taken to start process %t \n%r %s \nSize of response in bytes, including HTTP headers is %b\n%{Referer}i \nUser Agent %{User-Agent}i \nTime taken to serve the request, in milliseconds %D",
            ))
            .service(routes::user_routes::register)
            .service(routes::user_routes::login)
            .service(routes::user_routes::get_users)
            .service(routes::user_routes::get_user_by_id)
            .service(routes::user_routes::get_user_by_name)
            .service(routes::files_routes::upload_files)
    })
    .bind(format!(
        "{}:{}",
        env::var("HOST").unwrap(),
        env::var("PORT").unwrap()
    ))?
    .run()
    .await
}
