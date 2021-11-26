#![feature(proc_macro_hygiene, decl_macro)]

mod utils;
mod file;

#[macro_use]
extern crate rocket;

use rocket::data::Data;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::status::{self, NotFound};
use rocket::http::ContentType;

use rocket_dyn_templates::Template;
use filter::{self, Algorithms};

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

    let source = path.ok_or_else(|| NotFound(String::from("Could not save file")))?;
    let dest = file::get_new_image_file(&source, &algo_name)
        .map_err(|e| NotFound(e.get_error_string()))?;

    filter::run_algo(&source, &dest, algo).map_err(|e| NotFound(e.to_string()))?;
    NamedFile::open(&dest).await.map_err(|e| NotFound(e.to_string()))
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
        .mount("/style", FileServer::from("static/style"))
        .mount("/public", routes![index_public])
        .mount("/", routes![index, save, apply])
        .attach(Template::fairing())
        .launch()
        .await
}
