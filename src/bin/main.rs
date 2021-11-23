#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::error::Error;
use std::path::{Path, PathBuf};
use rocket::data::Data;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::status::{self, NotFound};
use rocket::http::ContentType;

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
    let path = match filter::run_algo(path, algo, algo_name) {
        Err(e) => Err(NotFound(e.get_error_string())),
        Ok(path) => Ok(path),
    }?;

    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

fn gen_html() {
    let mut html = String::from("
<head>
    <title>Multy</title>
</head>

<body>
    ");


    for entry in std::fs::read_dir("static/images").unwrap() {
        let osstr = entry.unwrap().file_name();
        let filename = osstr.to_str().unwrap();
        html.push_str(&format!("
    <pre>{}</pre>
    <img src=\"images/{}\">
    </br>
        ", filename, filename));
    }
    html.push_str("
</body>");

    std::fs::write("static/index.html", html.as_bytes()).unwrap();
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    gen_html();
    rocket::build()
        .mount("/public", FileServer::from("static"))
        //.mount("/public", FileServer::from("images"))
        .mount("/showimages", routes![files])
        .mount("/", routes![index, save, apply])
        .launch()
        .await?;

    Ok(())
}
