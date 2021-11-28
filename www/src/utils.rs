extern crate rocket_multipart_form_data;

use std::path::{
    Path, PathBuf,
};
use std::fs::{
    self, File,
};
use rocket::data::Data;
use rocket::http::ContentType;
use rocket::response::status;
use rand::{
    self, Rng, distributions::Alphanumeric,
};
use rocket_multipart_form_data::{
    MultipartFormDataOptions, MultipartFormData, MultipartFormDataField
};

pub async fn get_multipart_form_data(content_type: &ContentType, data: Data<'_>, fields: Vec<AllowedField<'_>>) -> MultipartFormData {
    let mut options = MultipartFormDataOptions::new();

    for field in fields {
        let multipart: MultipartFormDataField = field.into();
        options.allowed_fields.push(multipart);
    }

    MultipartFormData::parse(content_type, data, options).await.unwrap()
}

pub fn save_image(mut multipart_form_data: MultipartFormData) -> (status::Accepted<String>, Option<PathBuf>, String) {
    let algo = multipart_form_data.texts.remove("algorithm");
    let photo = multipart_form_data.files.remove("photo");
    let default_algo = String::from("flou_moyen");

    let algorithm = match algo {
        None => default_algo,
        Some(algo) => {
            if let Some(field) = algo.into_iter().next() {
                field.text
            } else {
                default_algo
            }
        }
    };

    if let Some(file_fields) = photo {
        let file_field = match file_fields.into_iter().next() {
            Some(field) => field,
            None => return (
                status::Accepted(Some(String::from("missing element under \"photo\" field"))),
                None, algorithm
            ),
        };

        use rocket_multipart_form_data::FileField;
        let FileField { file_name, path, .. } = file_field;

        let file_name = match file_name {
            Some(name) => name,
            None => {
                let mut rng = rand::thread_rng();
                (0..30).map(|_| rng.sample(Alphanumeric) as char).collect()
            }
        };

        let save_path = Path::new("static/images/").join(file_name);

        match File::create(&save_path) {
            Ok(_) => println!("created path"),
            Err(e) => return (status::Accepted(Some(format!("An Error occured while creating file: {}", e))), None, algorithm)
        };

        match fs::copy(path, &save_path) {
            Ok(_) => (status::Accepted(Some(String::from("Image saved"))), Some(save_path), algorithm),
            Err(e) => (status::Accepted(Some(format!("An Error occured while saving file: {}", e))), None, algorithm)
        }
    } else {
        (status::Accepted(Some(format!("An Error occured while parsing: {:?}", photo))), None, algorithm)
    }
}

pub enum AllowedField<'a>{
    File(&'a str),
    Text(&'a str),
}

impl<'a> From<AllowedField<'a>> for MultipartFormDataField<'a> {
    fn from(field: AllowedField<'a>) -> Self {
        match field {
            AllowedField::File(field_name) => MultipartFormDataField::file(field_name),
            AllowedField::Text(field_name) => MultipartFormDataField::text(field_name),
        }
    }
}
