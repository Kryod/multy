use std::{fmt::Display, error::Error, ffi::OsStr, path::Path, path::PathBuf};
use filter::ImageError;

#[derive(Debug)]
pub enum FilterError {
    DestImg(String),
    Image(String),
    Other(String),
}

impl FilterError {
    pub fn get_error_string(self) -> String {
        match self {
            FilterError::DestImg(s) |
            FilterError::Image(s) |
            FilterError::Other(s) => s,
        }
    }

    // pub fn get_ref_error_string(&self) -> &'_ str {
    //     match self {
    //         FilterError::DestImg(s) |
    //         FilterError::Image(s) |
    //         FilterError::Other(s) => s,
    //     }
    // }
}

impl From<ImageError> for FilterError {
    fn from(err: ImageError) -> Self {
        match err {
            ImageError::Decoding(e) => FilterError::Image(e.to_string()),
            ImageError::Encoding(e) => FilterError::Image(e.to_string()),
            ImageError::Parameter(e) => FilterError::Image(e.to_string()),
            ImageError::Limits(e) => FilterError::Image(e.to_string()),
            ImageError::Unsupported(e) => FilterError::Image(e.to_string()),
            ImageError::IoError(e) => FilterError::Image(e.to_string()),
        }
    }
}

impl Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FilterError {}

pub fn orig_filename_extension(path: &Path) -> Result<(&OsStr, &OsStr), FilterError> {
    let file_stem = path.file_stem();
    let extension = path.extension();

    match (file_stem, extension) {
        (Some(file_stem), Some(extension)) => Ok((file_stem, extension)),
        (None, Some(_)) => Err(FilterError::DestImg(format!("Path: {:?}, doesn't have a filename", path))),
        (Some(_), None) => Err(FilterError::DestImg(format!("Path: {:?}, doesn't have an extension", path))),
        (None, None) => Err(FilterError::DestImg(format!("Path: {:?}, doesn't have any filename or extension", path))),
    }
}

pub fn get_new_image_file(path: &Path, file_name_add: &str) -> Result<PathBuf, FilterError> {
    let (file_stem, extension) = orig_filename_extension(path)?;

    // prevent string realloc
    let mut new_path = String::with_capacity(file_stem.len() + file_name_add.len() + extension.len() + 2);

    new_path.push_str(file_stem.to_str().ok_or_else(||
        FilterError::Other(String::from("Failed to extract str from file_stem"))
    )?);
    new_path.push('_');
    new_path.push_str(file_name_add);
    new_path.push('.');
    new_path.push_str(extension.to_str().ok_or_else(||
        FilterError::Other(String::from("Failed to extract str from extension"))
    )?);

    let base_path = "static/images";
    let mut to_save = PathBuf::with_capacity(base_path.len() + new_path.len());

    to_save.push(base_path);
    to_save.push(new_path);

    Ok(to_save)
}
