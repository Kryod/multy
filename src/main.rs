#![feature(proc_macro_hygiene, decl_macro, test)]

#[macro_use]
extern crate rocket;

mod pixel_ops;
mod file;

pub mod utils;
pub mod filter;

use std::error::Error;
use std::path::{Path, PathBuf};
use rocket::data::Data;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::status;
use rocket::response::status::NotFound;
use rocket::http::ContentType;

use crate::filter::Algorithms;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("images/").join(file);
    NamedFile::open(&path).await.map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/save", data = "<data>")]
async fn save(content_type: &ContentType, data: Data<'_>) -> status::Accepted<String> {
    let multipart_form_data = utils::get_multipart_form_data(content_type, data).await;
    let (status, _, _) = utils::save_image(multipart_form_data);
    status
}

#[post("/apply", data = "<data>")]
async fn apply(content_type: &ContentType, data: Data<'_>) -> Result<NamedFile, NotFound<String>> {
    let multipart_form_data = utils::get_multipart_form_data(content_type, data).await;
    let (_, path, algo_name) = utils::save_image(multipart_form_data);
    let algo = Algorithms::get_algo(&algo_name);

    let path = path.ok_or_else(|| NotFound(String::from("Could not save file")))?;
    let path = match filter::run_algo(path, algo, algo_name) {
        Err(e) => Err(NotFound(e.get_error_string())),
        Ok(path) => Ok(path),
    }?;

    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
        .mount("/public", FileServer::from("images"))
        .mount("/showimages", routes![files])
        .mount("/", routes![index, save, apply])
        .launch()
        .await?;

    Ok(())
}
