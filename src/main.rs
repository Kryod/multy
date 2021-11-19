#![feature(proc_macro_hygiene, decl_macro, test)]

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

pub enum Algorithms {
    FlouMoyen,
    Erosion
}

impl Algorithms {
    pub fn get_algo(s: &str) -> Self {
        match s {
            "flou_moyen" => Self::FlouMoyen,
            "erosion" => Self::Erosion,
            _ => Self::FlouMoyen
        }
    }

    /*pub fn apply(&self, path: PathBuf, radius: u32) -> Result<PathBuf, FilterError>{
        match self {
            Algorithms::FlouMoyen => filter::flou_moyen(path, radius),
            Algorithms::Erosion => filter::erosion(path),
        }
    }*/
}

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

#[post("/floumoyen", data = "<data>")]
async fn flou_moyen(content_type: &ContentType, data: Data<'_>) -> Result<NamedFile, NotFound<String>> {
    let multipart_form_data = utils::get_multipart_form_data(content_type, data).await;

    let (_, path, algo) = utils::save_image(multipart_form_data);

    let algo_name = algo.clone();
    let algo = Algorithms::get_algo(&algo);

    if let None = path {
        return Err(NotFound(String::from("Could not save file")));
    }
    //let path = filter::flou_moyen(path.unwrap(), 2);
    let path = filter::run_algo(path.unwrap(), algo, algo_name);
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
