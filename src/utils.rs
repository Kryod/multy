use std::fs;
use std::fs::File;
use std::path::*;
use rocket::data::Data;
use rocket::response::status;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::http::ContentType;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, FileField};


pub fn get_multipart_form_data(content_type: &ContentType, data: Data) -> MultipartFormData {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::file("photo").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap());
    MultipartFormData::parse(content_type, data, options).unwrap()
}

pub fn save_image(multipart_form_data: MultipartFormData) -> (status::Accepted<String>, Option<PathBuf>) {
    let photo = multipart_form_data.files.get("photo");

    if let Some(photo) = photo {
        match photo {
            FileField::Single(file) => {
                let file_name = &file.file_name;
                let mut unwraped_file_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
                match file_name {
                    Some(name) => unwraped_file_name = name.clone(),
                    None => {}
                };
                let path = &file.path;
                let save_path = Path::new("images/").join(unwraped_file_name);
                match File::create(&save_path) {
                    Ok(_) => println!("created path"),
                    Err(e) => return (status::Accepted(Some(format!("An Error occured while creating file: {}", e))), None)
                };
                match fs::copy(path, &save_path) {
                    Ok(_) => (status::Accepted(Some(format!("Image saved"))), Some(save_path.clone())),
                    Err(e) => (status::Accepted(Some(format!("An Error occured while saving file: {}", e))), None)
                }
                // You can now deal with the uploaded file. The file will be delete automatically when the MultipartFormData instance is dropped. If you want to handle that file by your own, instead of killing it, just remove it out from the MultipartFormData instance.
            },
            FileField::Multiple(_files) => {
                (status::Accepted(Some(format!("Image saved"))), None)
                // Because we only put one "photo" field to the allowed_fields, this arm will not be matched.
            }

        }
    } else {
        (status::Accepted(Some(format!("An Error occured while parsing: {:?}", photo))), None)
    }
}