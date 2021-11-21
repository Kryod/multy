use image::{ImageBuffer, ImageError, Rgba};
use std::{error::Error, ffi::OsStr, fmt::Display, path::PathBuf};

use crate::Algorithms;
type Buffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug)]
pub enum FilterError {
    DestImgError(String),
    ImageError(String),
    OtherError(String),
}

impl FilterError {
    pub fn get_error_string(self) -> String {
        match self {
            FilterError::DestImgError(s) |
            FilterError::ImageError(s) |
            FilterError::OtherError(s) => s,
        }
    }

    pub fn get_ref_error_string<'a>(&'a self) -> &'a str {
        match self {
            FilterError::DestImgError(ref s) |
            FilterError::ImageError(ref s) |
            FilterError::OtherError(ref s) => s,
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

pub fn run_algo(path: PathBuf, algo: Algorithms, algo_name: String) -> Result<PathBuf, FilterError> {
    let mut fname = String::with_capacity(algo_name.len() + 2);
    fname.push('_');
    fname.push_str(&algo_name);
    fname.push('.');

    let dest = get_new_image_file(&path, &fname)?;
    let img = image::open(path)?.into_rgba8();
    let radius = 2;

    let buffer = match algo {
        Algorithms::FlouMoyen => flou_moyen(&img, radius),
        Algorithms::Erosion => erosion(&img, radius),
        Algorithms::Dilatation => dilatation(&img, radius),
    };

    buffer.save(&dest)?;
    Ok(dest)
}

pub fn orig_filename_extension(path: &PathBuf) -> Result<(&OsStr, &OsStr), FilterError> {
    let file_stem = path.file_stem();
    let extension = path.extension();

    match (file_stem, extension) {
        (Some(file_stem), Some(extension)) => Ok((file_stem, extension)),
        (None, Some(_)) => Err(FilterError::DestImgError(format!("Path: {:?}, doesn't have a filename", path))),
        (Some(_), None) => Err(FilterError::DestImgError(format!("Path: {:?}, doesn't have an extension", path))),
        (None, None) => Err(FilterError::DestImgError(format!("Path: {:?}, doesn't have any filename or extension", path))),
    }
}

pub fn flou_moyen(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(&img, radius, [0; 4],
        |pix, sum| {
            sum[0] += pix[0] as u32;
            sum[1] += pix[1] as u32;
            sum[2] += pix[2] as u32;
            sum[3] += pix[3] as u32;
        },
        |col, sum| {
            sum[0] += col[0];
            sum[1] += col[1];
            sum[2] += col[2];
            sum[3] += col[3];
        },
        |sum, neighbours| [
            (sum[0] / neighbours) as u8,
            (sum[1] / neighbours) as u8,
            (sum[2] / neighbours) as u8,
            (sum[3] / neighbours) as u8,
        ],
    )
}

pub fn erosion(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(&img, radius, [u8::MAX; 4],
        |pix, min| {
            if min[0] > pix[0] { min[0] = pix[0] }
            if min[1] > pix[1] { min[1] = pix[1] }
            if min[2] > pix[2] { min[2] = pix[2] }
            if min[3] > pix[3] { min[3] = pix[3] }
        },
        |col, min| {
            if min[0] > col[0] { min[0] = col[0] }
            if min[1] > col[1] { min[1] = col[1] }
            if min[2] > col[2] { min[2] = col[2] }
            if min[3] > col[3] { min[3] = col[3] }
        },
        |min, _| min
    )
}

pub fn dilatation(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(&img, radius, [u8::MIN; 4],
        |pix, max| {
            if max[0] < pix[0] { max[0] = pix[0] }
            if max[1] < pix[1] { max[1] = pix[1] }
            if max[2] < pix[2] { max[2] = pix[2] }
            if max[3] < pix[3] { max[3] = pix[3] }
        },
        |col, max| {
            if max[0] < col[0] { max[0] = col[0] }
            if max[1] < col[1] { max[1] = col[1] }
            if max[2] < col[2] { max[2] = col[2] }
            if max[3] < col[3] { max[3] = col[3] }
        },
        |max, _| max
    )
}

pub fn median(img: &Buffer, radius: u32) -> Buffer {
    let capacity = (radius * 2 + 1).pow(2) as usize;
    let accumulator = Vec::with_capacity(capacity);

    compute_buffer(&img, radius, accumulator,
        |pix, vec| {
            let brightness = pix[0] / 4 + pix[1] / 4 + pix[2] / 4 + pix[3] / 4;
            vec.push((brightness, *pix));
        },
        |col, vec| {
            vec.extend(col);
        },
        |mut vec, neighbours| {
            vec.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
            vec[(neighbours / 2) as usize].1
        }
    )
}

pub fn get_new_image_file(path: &PathBuf, file_name_add: &str) -> Result<PathBuf, FilterError> {
    let (file_stem, extension) = orig_filename_extension(&path)?;

    // prevent string realloc
    let mut new_path = String::with_capacity(file_stem.len() + file_name_add.len() + extension.len());

    new_path.push_str(file_stem.to_str().ok_or_else(||
        FilterError::OtherError(String::from("Failed to extract str from file_stem"))
    )?);
    new_path.push_str(file_name_add);
    new_path.push_str(extension.to_str().ok_or_else(||
        FilterError::OtherError(String::from("Failed to extract str from extension"))
    )?);

    let base_path = "images";
    let mut to_save = PathBuf::with_capacity(base_path.len() + new_path.len());

    to_save.push(base_path);
    to_save.push(new_path);

    Ok(to_save)
}

fn compute_buffer<T>(
    img: &Buffer,
    radius: u32,
    accumulator: T,
    reduce: fn(&[u8; 4], &mut T),
    concat: fn(&T, &mut T),
    average: fn(T, u32) -> [u8; 4],
) -> Buffer where T: Clone {
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
            let mut acc = accumulator.clone();

            for neighbour_y in y_min..y_max {
                let pix = &img.get_pixel(neighbour_x, neighbour_y).0;
                reduce(pix, &mut acc);
            }

            partial_blur.push_back(acc);
        }

        // compute every Pixels
        for x in 0..width {
            let mut acc = accumulator.clone();
            let neighbours = (y_max - y_min) * partial_blur.len() as u32;

            for col in &partial_blur {
                concat(col, &mut acc);
            }

            let avg = average(acc, neighbours);
            buffer.put_pixel(x, y, image::Rgba(avg));

            // compute next partial blur row
            let x_target = x.saturating_add(radius + 1);
            if x_target < width {
                let mut acc = accumulator.clone();

                for neighbour_y in y_min..y_max {
                    let pix = &img.get_pixel(x_target, neighbour_y).0;
                    reduce(pix, &mut acc);
                }

                partial_blur.push_back(acc);
            }

            if x > radius {
                partial_blur.pop_front();
            }
        }
    }

    buffer
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use std::{error::Error, path::PathBuf};
    use crate::filter::{
        get_new_image_file, flou_moyen, erosion, dilatation, median
    };

    const RADIUS: u32 = 2;
    const IMG: &str = "images/lena.jpg";

    #[test]
    fn test_flou_moyen() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let algo_name = "flou_moyen";

        let mut fname = String::with_capacity(algo_name.len() + 2);
        fname.push('_');
        fname.push_str(&algo_name);
        fname.push('.');

        let dest = get_new_image_file(&path, &fname)?;
        let img = image::open(path)?.into_rgba8();

        let start = std::time::Instant::now();
        let buffer = flou_moyen(&img, RADIUS);
        let elapsed = start.elapsed().as_millis();
        println!("flou_moyen: {} ms", elapsed);

        buffer.save(&dest)?;
        Ok(())
    }

    #[bench]
    fn bench_flou_moyen(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let img = image::open(path)?.into_rgba8();

        b.iter(|| flou_moyen(&img, RADIUS));
        Ok(())
    }

    #[test]
    fn test_erosion() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let algo_name = "erosion";

        let mut fname = String::with_capacity(algo_name.len() + 2);
        fname.push('_');
        fname.push_str(&algo_name);
        fname.push('.');

        let dest = get_new_image_file(&path, &fname)?;
        let img = image::open(path)?.into_rgba8();

        let start = std::time::Instant::now();
        let buffer = erosion(&img, RADIUS);
        let elapsed = start.elapsed().as_millis();
        println!("erosion: {} ms", elapsed);

        buffer.save(&dest)?;
        Ok(())
    }

    #[bench]
    fn bench_erosion(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let img = image::open(path)?.into_rgba8();

        b.iter(|| erosion(&img, RADIUS));
        Ok(())
    }

    #[test]
    fn test_dilatation() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let algo_name = "dilatation";

        let mut fname = String::with_capacity(algo_name.len() + 2);
        fname.push('_');
        fname.push_str(&algo_name);
        fname.push('.');

        let dest = get_new_image_file(&path, &fname)?;
        let img = image::open(path)?.into_rgba8();

        let start = std::time::Instant::now();
        let buffer = dilatation(&img, RADIUS);
        let elapsed = start.elapsed().as_millis();
        println!("dilatation: {} ms", elapsed);

        buffer.save(&dest)?;
        Ok(())
    }

    #[bench]
    fn bench_dilatation(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let img = image::open(path)?.into_rgba8();

        b.iter(|| dilatation(&img, RADIUS));
        Ok(())
    }

    #[test]
    fn test_median() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let algo_name = "median";

        let mut fname = String::with_capacity(algo_name.len() + 2);
        fname.push('_');
        fname.push_str(&algo_name);
        fname.push('.');

        let dest = get_new_image_file(&path, &fname)?;
        let img = image::open(path)?.into_rgba8();

        let start = std::time::Instant::now();
        let buffer = median(&img, RADIUS);
        let elapsed = start.elapsed().as_millis();
        println!("median: {} ms", elapsed);

        buffer.save(&dest)?;
        Ok(())
    }

    #[bench]
    fn bench_median(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let img = image::open(path)?.into_rgba8();

        b.iter(|| median(&img, RADIUS));
        Ok(())
    }
}
