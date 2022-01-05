use std::env;
use std::fs;
use std::io::Write;

use actix_multipart::Multipart;

use actix_web::post;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;

use crate::middelware::auth;
use crate::models::success;
use futures::TryStreamExt;
use uuid::Uuid;

#[post("/uploadFiles", wrap = "auth::AuthorizationService")]
pub async fn upload_files(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let path = env::var("UPLOADPATH").unwrap();
    println!("path: {}", path);
    while let Some(mut item) = payload.try_next().await? {
        let content_disposition = item
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;
        let filename = content_disposition.get_filename().map_or_else(
            || Uuid::new_v4().to_string(),
            |f| -> String { sanitize_filename::sanitize(f) },
        );
        println!("Filename: {}", filename);
        if !fs::metadata(env::var("UPLOADPATH").unwrap()).is_ok() {
            fs::create_dir_all(env::var("UPLOADPATH").unwrap()).unwrap();
        }
        let filepath = format!("{}/{}", path, filename);

        let mut f = web::block(|| std::fs::File::create(filepath)).await?;

        while let Some(chunk) = item.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().json(success::Success {
        message: "File uploaded successfully".to_string(),
    }))
}
