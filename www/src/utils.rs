extern crate rocket_multipart_form_data;

use filter::Algorithms;

use std::fs::{
    self, File,
};
use std::path::{
    Path, PathBuf,
};
use rocket::{
    data::Data, http::ContentType,
};
use rand::{
    self, Rng, distributions::Alphanumeric,
};
use rocket_multipart_form_data::{
    MultipartFormDataOptions, MultipartFormData, MultipartFormDataField,
};

pub async fn get_multipart_form_data(content_type: &ContentType, data: Data<'_>, fields: Vec<AllowedField<'_>>) -> MultipartFormData {
    let mut options = MultipartFormDataOptions::new();

    for field in fields {
        let multipart = field.into();
        options.allowed_fields.push(multipart);
    }

    MultipartFormData::parse(content_type, data, options).await.unwrap()
}

pub fn save_image(multipart_form_data: &mut MultipartFormData) -> Result<PathBuf, String> {
    if let Some(file_fields) = multipart_form_data.files.remove("photo") {
        let file_field = match file_fields.into_iter().next() {
            Some(field) => field,
            None => return Err(String::from("missing element under \"photo\" field")),
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
            Err(e) => return Err(format!("An Error occured while creating file: {}", e)),
            Ok(_) => {},
        };

        match fs::copy(path, &save_path) {
            Err(e) => Err(format!("An Error occured while saving file: {}", e)),
            Ok(_) => Ok(save_path),
        }
    } else {
        Err(String::from("Missing \"photo\" field in form."))
    }
}

pub fn get_algo(multipart_form_data: &mut MultipartFormData) -> Result<(Algorithms, String), String> {
    let algorithm = match multipart_form_data.texts.remove("algorithm") {
        None => Err(String::from("Missing \"algorithm\" field.")),
        Some(algo) => {
            if let Some(field) = algo.into_iter().next() {
                Ok(field.text)
            } else {
                Err(String::from("Missing value in \"algorithm\" field."))
            }
        }
    }?;

    let mut algo = Algorithms::try_from(algorithm.as_str())?;

    if algo.need_radius() {
        let radius = match multipart_form_data.texts.remove("radius") {
            None => Err(format!("{}: missing \"radius\" field", algo)),
            Some(rad) => {
                if let Some(field) = rad.into_iter().next() {
                    field.text.parse::<u32>().map_err(|e| format!("{}: \"radius\" -> {}", algo, e))
                } else {
                    Err(format!("{}: missing value in \"radius\" field", algo))
                }
            }
        }?;

        algo.set_radius(radius);
    }

    // Algorithm enum + original name
    // ex: (Algorithm::Blur(2), "blur")
    Ok((algo, algorithm))
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
