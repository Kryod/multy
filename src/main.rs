#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod utils;
pub mod filter;

use std::error::Error;
use std::path::{Path, PathBuf};
use rocket::data::Data;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::status;
use rocket::response::status::NotFound;
use rocket::http::ContentType;


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

    let (status, _path) = utils::save_image(multipart_form_data);
    status
}

#[post("/floumoyen", data = "<data>")]
async fn flou_moyen(content_type: &ContentType, data: Data<'_>) -> Result<NamedFile, NotFound<String>> {
    let multipart_form_data = utils::get_multipart_form_data(content_type, data);

    let (_status, path) = utils::save_image(multipart_form_data.await);

    if let None = path {
        return Err(NotFound(String::from("Could not save file")));
    }
    let path = filter::flou_moyen(path.unwrap(), 2);
    if let Err(e) = path {
        return Err(NotFound(e.get_error_string()));
    }

    NamedFile::open(&path.unwrap()).await.map_err(|e| NotFound(e.to_string()))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
    .mount("/public", FileServer::from("images"))
    .mount("/showimages", routes![files])
    .mount("/", routes![index, save, flou_moyen])
    .launch().await?;

    Ok(())
}
