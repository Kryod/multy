pub mod median_blur;
pub mod min_max;
pub mod dilate;
pub mod erode;
pub mod blur;

use image::{ImageBuffer, Rgba};
use std::path::Path;

pub type Buffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub enum Algorithms {
    Blur,
    Dilate,
    Erode,
    MedianBlur,
    MinMax,
}

impl Algorithms {
    pub fn get_algo(algo_name: &str) -> Option<Self> {
        match algo_name {
            "dilate" => Some(Self::Dilate),
            "erode" => Some(Self::Erode),
            "median_blur" => Some(Self::MedianBlur),
            "min_max" => Some(Self::MinMax),
            "blur" => Some(Self::Blur),
            _ => None,
        }
    }
}

pub fn run_algo(source: &Path, dest: &Path, algo: Algorithms) -> Result<(), image::ImageError> {
    let img = image::open(source)?.into_rgba8();
    let radius = 2;

    let buffer = match algo {
        Algorithms::Blur => blur::blur(&img, radius),
        Algorithms::Dilate => dilate::dilate(&img, radius),
        Algorithms::Erode => erode::erode(&img, radius),
        Algorithms::MedianBlur => median_blur::median_blur(&img, radius),
        Algorithms::MinMax => min_max::min_max(&img, radius),
    };

    buffer.save(&dest)
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
