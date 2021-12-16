pub mod adaptive_threshold;
pub mod local_contrast;
pub mod median_blur;
pub mod compare;
pub mod min_max;
pub mod dilate;
pub mod erode;
pub mod blur;

use image::{ImageBuffer, Rgba};
use std::convert::TryFrom;
use std::path::Path;

pub type Buffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub enum Algorithms {
    AdaptiveThreshold(u32, i32),
    Blur(u32),
    Dilate(u32),
    Erode(u32),
    LocalContrast(u32, i32),
    MedianBlur(u32),
    MinMax(u32),
}

impl Algorithms {
    pub fn set_radius(&mut self, radius: u32) {
        match self {
            Self::AdaptiveThreshold(r, _) |
            Self::Blur(r) |
            Self::Dilate(r) |
            Self::Erode(r) |
            Self::LocalContrast(r, _) |
            Self::MedianBlur(r) |
            Self::MinMax(r) => *r = radius,
        }
    }

    pub fn set_factor(&mut self, factor: i32) {
        match self {
            Self::AdaptiveThreshold(_, f) |
            Self::LocalContrast(_, f) => *f = factor,
            _ => {}
        }
    }

    pub fn need_radius(&self) -> bool {
        true
    }

    pub fn need_factor(&self) -> bool {
        matches!(self, Self::AdaptiveThreshold(..) | Self::LocalContrast(..))
    }
}

impl TryFrom<&str> for Algorithms {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "adaptive_threshold" => Ok(Self::AdaptiveThreshold(0, 0)),
            "blur" => Ok(Self::Blur(0)),
            "dilate" => Ok(Self::Dilate(0)),
            "erode" => Ok(Self::Erode(0)),
            "local_contrast" => Ok(Self::LocalContrast(0, 0)),
            "median_blur" => Ok(Self::MedianBlur(0)),
            "min_max" => Ok(Self::MinMax(0)),
            unknown => Err(format!("\"{}\" isn't a valid algorithm name.", unknown)),
        }
    }
}

impl std::fmt::Display for Algorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let algo_name = match self {
            Algorithms::AdaptiveThreshold(..) => "adaptive threshold",
            Algorithms::Blur(_) => "blur",
            Algorithms::Dilate(_) => "dilate",
            Algorithms::Erode(_) => "erode",
            Algorithms::LocalContrast(..) => "local contrast",
            Algorithms::MedianBlur(_) => "median blur",
            Algorithms::MinMax(_) => "min max",
        };

        f.write_str(algo_name)
    }
}

pub fn run_algo(source: &Path, dest: &Path, algo: Algorithms) -> Result<(), image::ImageError> {
    let img = image::open(source)?.into_rgba8();

    let buffer = match algo {
        Algorithms::AdaptiveThreshold(radius, f) => adaptive_threshold::adaptive_threshold(&img, radius, f),
        Algorithms::Blur(radius) => blur::blur(&img, radius),
        Algorithms::Dilate(radius) => dilate::dilate(&img, radius),
        Algorithms::Erode(radius) => erode::erode(&img, radius),
        Algorithms::LocalContrast(radius, factor) => local_contrast::local_contrast(&img, radius, factor),
        Algorithms::MedianBlur(radius) => median_blur::median_blur(&img, radius),
        Algorithms::MinMax(radius) => min_max::min_max(&img, radius),
    };

    buffer.save(&dest)
}

pub fn compare_images(left: &Path, right: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let left = image::open(left)?.into_rgba8();
    let right = image::open(right)?.into_rgba8();

    compare::compare(&left, &right)?.save(dest)?;
    Ok(())
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
