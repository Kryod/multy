#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod utils;
pub mod filter;

use std::path::*;
use rocket::data::Data;
use rocket::response::status;
use rocket_contrib::serve::StaticFiles;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;
use rocket::http::ContentType;


#[get("/<file..>")]
fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("images/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/save", data = "<data>")]
fn save(content_type: &ContentType, data: Data) -> status::Accepted<String> {
    let multipart_form_data = utils::get_multipart_form_data(content_type, data);

    let (status, _path) = utils::save_image(multipart_form_data);
    status
}

#[post("/floumoyen", data = "<data>")]
fn flou_moyen(content_type: &ContentType, data: Data) -> status::Accepted<String> {
    let multipart_form_data = utils::get_multipart_form_data(content_type, data);

    let (status, _path) = utils::save_image(multipart_form_data);

    filter::flou_moyen();
    status
}

fn main() {
    rocket::ignite()
    .mount("/public", StaticFiles::from("images"))
    .mount("/showimages", routes![files])
    .mount("/", routes![index, save, flou_moyen]).launch();
}
