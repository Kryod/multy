#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::data::Data;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::status::{self, NotFound};
use rocket::http::ContentType;

use rocket_dyn_templates::Template;

use multy::filter::{self, Algorithms};
use multy::utils;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/images/").join(file);
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
    let path = filter::run_algo(path, algo, algo_name).map_err(|e| NotFound(e.get_error_string()))?;
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[get("/")]
fn index_public() -> Template {
    #[derive(serde::Serialize)]
    struct Data { images: Vec<String> }
    let mut vec = Vec::with_capacity(10);

    for entry in std::fs::read_dir("static/images").unwrap() {
        let full_name = entry.unwrap().file_name();
        vec.push(full_name.to_str().unwrap().to_owned());
    }

    Template::render("index", Data {
        images: vec,
    })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/public", FileServer::from("static/images"))
        .mount("/public", routes![index_public])
            .attach(Template::fairing())
        .mount("/showimages", routes![files])
        .mount("/", routes![index, save, apply])
        .launch()
        .await
}