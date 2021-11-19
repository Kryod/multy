use std::{error::Error, ffi::OsStr, fmt::Display, path::PathBuf};
use image::{GenericImageView, ImageError, Pixel};

#[derive(Debug)]
pub enum FilterError {
    DestImgError(String),
    ImageError(String),
    OtherError(String)
}

impl FilterError {
    pub fn get_error_string(&self) -> String {
        match self {
            FilterError::DestImgError(s) => s.clone(),
            FilterError::ImageError(s) => s.clone(),
            FilterError::OtherError(s) => s.clone(),
        }
    }
}

impl From<ImageError> for FilterError {
    fn from(err: ImageError) -> Self {
        match err {
            ImageError::Decoding(e) => FilterError::ImageError(e.to_string()),
            ImageError::Encoding(e) => FilterError::ImageError(e.to_string()),
            ImageError::Parameter(e) => FilterError::ImageError(e.to_string()),
            ImageError::Limits(e) => FilterError::ImageError(e.to_string()),
            ImageError::Unsupported(e) => FilterError::ImageError(e.to_string()),
            ImageError::IoError(e) => FilterError::ImageError(e.to_string()),
        }

    }
}

impl Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FilterError {}

pub fn orig_filename_extension(path: &PathBuf) -> Result<(&OsStr, &OsStr), FilterError> {
    let file_stem = path.file_stem();
    let extension = path.extension();

    if let None = file_stem {
        return Err(FilterError::DestImgError(format!("Path: {:?}, doesn't have a filename", path)))
    }
    if let None = extension {
        return Err(FilterError::DestImgError(format!("Path: {:?}, doesn't have an extension", path)))
    }

    Ok((file_stem.unwrap(), extension.unwrap()))
}

pub fn get_new_image_file(path: &PathBuf, file_name_add: &str) -> Result<PathBuf, FilterError> {

    let (file_stem, extension) = orig_filename_extension(&path)?;


    // prevent string realloc
    let mut new_path = String::with_capacity(
        file_stem.len() + file_name_add.len() + extension.len()
    );

    new_path.push_str(file_stem.to_str().unwrap());
    new_path.push_str(file_name_add);
    new_path.push_str(extension.to_str().unwrap());

    let base_path = "images";
    let mut to_save = PathBuf::with_capacity(
        base_path.len() + new_path.len()
    );

    to_save.push(base_path);
    to_save.push(new_path);

    Ok(to_save)
}

pub fn flou_moyen(path: PathBuf, radius: u32) -> Result<PathBuf, FilterError> {
    let to_save = get_new_image_file(&path, "_flou_moyen.")?;

    let img = image::open(path).unwrap();

    let width = img.width();
    let height = img.height();
    let mut buffer = image::ImageBuffer::new(width, height);
    let mut partial_blur = std::collections::VecDeque::with_capacity(radius as usize * 2 + 2);

    for y in 0..height {
        let y_max = y.saturating_add(radius + 1).min(height);
        let y_min = y.saturating_sub(radius);
        partial_blur.clear();

        // init partial blur
        for neighbour_x in 0..=radius {
            let mut sum = [0u32; 4];

            for neighbour_y in y_min..y_max {
                let pix = &img.get_pixel(neighbour_x, neighbour_y).0;
                sum[0] += pix[0] as u32;
                sum[1] += pix[1] as u32;
                sum[2] += pix[2] as u32;
                sum[3] += pix[3] as u32;
            }

            partial_blur.push_back(sum);
        }

        // compute every Pixels
        for x in 0..width {
            let mut sum = [0; 4];
            let neighbours = (y_max - y_min) * partial_blur.len() as u32;

            partial_blur.iter().for_each(|col| {
                sum[0] += col[0] as u32;
                sum[1] += col[1] as u32;
                sum[2] += col[2] as u32;
                sum[3] += col[3] as u32;
            });

            let avg = [(sum[0] / neighbours) as u8, (sum[1] / neighbours) as u8, (sum[2] / neighbours) as u8, (sum[3] / neighbours) as u8];
            buffer.put_pixel(x, y, image::Rgba(avg));

            // compute next partial blur row
            let x_target = x.saturating_add(radius + 1);
            if x_target < width {
                let mut sum = [0u32; 4];

                for neighbour_y in y_min..y_max {
                    let pix = &img.get_pixel(x_target, neighbour_y).0;
                    sum[0] += pix[0] as u32;
                    sum[1] += pix[1] as u32;
                    sum[2] += pix[2] as u32;
                    sum[3] += pix[3] as u32;
                }

                partial_blur.push_back(sum);
            }

            if x > radius {
                partial_blur.pop_front();
            }
        }
    }

    buffer.save(&to_save).unwrap();
    Ok(to_save)
}

pub fn erosion(path: PathBuf) -> Result<PathBuf, FilterError> {

    let to_save = get_new_image_file(&path, "_erosion.")?;
    let img = image::open(path).unwrap();
    let mut buffer = image::ImageBuffer::new(img.width(), img.height());

    let radius = 2;

    let width = img.width();
    let height = img.height();

    for x in 0..width {
        for y in 0..height {

            let mut min = img.get_pixel(x, y).channels4();
            for neighbour_x in (x.saturating_sub(radius))..(x.saturating_add(radius + 1)) {
                for neighbour_y in (y.saturating_sub(radius))..(y.saturating_add(radius + 1)) {
                    if !(neighbour_x >= width || neighbour_y >= height) {
                        let p = img.get_pixel(neighbour_x, neighbour_y).channels4();
                        if min.0 > p.0 {
                            min.0 = p.0;
                        }
                        if min.1 > p.1 {
                            min.1 = p.1;
                        }
                        if min.2 > p.2 {
                            min.2 = p.2;
                        }
                        if min.3 > p.3 {
                            min.3 = p.3;
                        }
                    }
                }
            }

            buffer.put_pixel(x, y, image::Rgba([min.0, min.1, min.2, min.3]));
        }
    }
    println!("{:?}", &to_save);
    buffer.save(&to_save).unwrap();

    Ok(to_save)
}

#[cfg(test)]
mod tests {

    use std::{error::Error, path::PathBuf};

    #[test]
    fn test_flou_moyen() -> Result<(), Box<dyn Error>> {
        let start = std::time::Instant::now();
        super::flou_moyen(PathBuf::from("images/lena.jpg"), 2)?;

        let elapsed = start.elapsed().as_millis();
        println!("flou_moyen: {} ms", elapsed);
        Ok(())
    }

    #[test]
    fn test_erosion() -> Result<(), Box<dyn Error>>{
        let start = std::time::Instant::now();
        super::erosion(PathBuf::from("images/lena.jpg"))?;

        let elapsed = start.elapsed().as_millis();
        println!("erosion: {} ms", elapsed);
        Ok(())
    }   

}
