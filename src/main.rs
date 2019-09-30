#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::path::*;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;

#[get("/<file..>")]
fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("images/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, files]).launch();
}
