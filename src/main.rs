mod models;
mod routes;
mod schema;
mod utils;
extern crate dotenv;
extern crate uuid;
#[macro_use]
extern crate diesel;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}
#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection = utils::database::create_database_pool();
    let subscriber = get_subscriber("app".into(), "info".into());
    init_subscriber(subscriber);
    HttpServer::new(move || {
        App::new()
            .data(connection.clone())
            .wrap(TracingLogger)
            .service(hello)
            .service(hello_name)
            .service(routes::user_routes::register)
            .service(routes::user_routes::login)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
