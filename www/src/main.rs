#![feature(proc_macro_hygiene, decl_macro)]

mod utils;
mod file;

#[macro_use]
extern crate rocket;

use rocket::data::Data;
use rocket::response::status;
use rocket::http::ContentType;
use rocket::fs::{FileServer, NamedFile};

use rocket_dyn_templates::Template;
use filter;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/save", data = "<data>")]
async fn save(content_type: &ContentType, data: Data<'_>) -> Result<status::Created<String>, status::BadRequest<String>> {
    let fields = vec![
        utils::AllowedField::File("photo"),
    ];

    let mut multipart_form_data = utils::get_multipart_form_data(content_type, data, fields).await;
    let location = utils::save_image(&mut multipart_form_data).map_err(|e|
        status::BadRequest(Some(e))
    )?;

    let url = "/public/";
    let file = location
        .file_name().unwrap()
        .to_str().unwrap();

    let mut location = String::with_capacity(url.len() + file.len());
    location.push_str(url);
    location.push_str(file);

    Ok(status::Created::new(location))
}

#[post("/apply", data = "<data>")]
async fn apply(content_type: &ContentType, data: Data<'_>) -> Result<NamedFile, status::BadRequest<String>> {
    let fields = vec![
        utils::AllowedField::Text("algorithm"),
        utils::AllowedField::Text("radius"),
        // utils::AllowedField::Text("factor"),
        utils::AllowedField::File("photo"),
    ];

    let mut multipart_form_data = utils::get_multipart_form_data(content_type, data, fields).await;
    let source = utils::save_image(&mut multipart_form_data).map_err(|e|
        status::BadRequest(Some(e))
    )?;
    let (algo, name) = utils::get_algo(&mut multipart_form_data).map_err(|e|
        status::BadRequest(Some(e))
    )?;

    let dest = file::get_new_image_file(source.as_path(), &name)
        .map_err(|e| status::BadRequest(Some(e.get_error_string())))?;

    filter::run_algo(&source, &dest, algo).map_err(|e|
        status::BadRequest(Some(e.to_string()))
    )?;

    NamedFile::open(&dest).await.map_err(|e|
        status::BadRequest(Some(e.to_string()))
    )
}

#[get("/")]
fn index_public() -> Template {
    #[derive(serde::Serialize)]
    struct Data { images: Vec<String> }
    let mut vec = Vec::with_capacity(10);

    for entry in std::fs::read_dir("static/images").unwrap() {
        let full_name = entry.unwrap().file_name();
        vec.push(full_name.into_string().unwrap());
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
