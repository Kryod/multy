extern crate rocket_multipart_form_data;

use std::fs;
use std::fs::File;
use std::path::*;
use rocket::data::Data;
use rocket::http::ContentType;
use rocket::response::status;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField};


pub async fn get_multipart_form_data(content_type: &ContentType, data: Data<'_>) -> MultipartFormData {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::file("photo").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap());
    MultipartFormData::parse(content_type, data, options).await.unwrap()
}

pub fn save_image(multipart_form_data: MultipartFormData) -> (status::Accepted<String>, Option<PathBuf>, String) {
    let photo = multipart_form_data.files.get("photo");
    let algo = multipart_form_data.texts.get("algorithm");
    
    let algorithm = match algo {
        Some(alg) => {
            let al = &alg[0];
            al.text.clone()
        },
        None => String::from("flou_moyen")
    };

    if let Some(file_fields) = photo {
        let file_field = &file_fields[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

        //let _content_type = &file_field.content_type;
        let file_name = &file_field.file_name;
        let path = &file_field.path;

        let mut rng = thread_rng();
        let mut unwraped_file_name: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(30)
            .collect();
        match file_name {
            Some(name) => unwraped_file_name = name.clone(),
            None => {}
        };
        let save_path = Path::new("images/").join(unwraped_file_name);
        match File::create(&save_path) {
            Ok(_) => println!("created path"),
            Err(e) => return (status::Accepted(Some(format!("An Error occured while creating file: {}", e))), None, algorithm)
        };
        match fs::copy(path, &save_path) {
            Ok(_) => (status::Accepted(Some(format!("Image saved"))), Some(save_path.clone()), algorithm),
            Err(e) => (status::Accepted(Some(format!("An Error occured while saving file: {}", e))), None, algorithm)
        }

    } else {
        (status::Accepted(Some(format!("An Error occured while parsing: {:?}", photo))), None, algorithm)
    }
}