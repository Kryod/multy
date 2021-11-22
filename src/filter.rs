use image::{ImageBuffer, Rgba};
use std::path::PathBuf;

type Buffer = ImageBuffer<Rgba<u8>, Vec<u8>>;
use crate::file::{FilterError, get_new_image_file};

pub enum Algorithms {
    FlouMoyen,
    Erosion,
    Dilatation,
    Median,
}

impl Algorithms {
    pub fn get_algo(s: &str) -> Self {
        match s {
            "erosion" => Self::Erosion,
            "dilatation" => Self::Dilatation,
            "median" => Self::Median,
            _ /* flou_moyen */ => Self::FlouMoyen,
        }
    }
}

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
        Algorithms::Median => median(&img, radius),
    };

    buffer.save(&dest)?;
    Ok(dest)
}

pub fn flou_moyen(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(img, radius, [0; 4],
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

pub fn optimized_blur(img: &Buffer, radius: u32) -> Buffer {
    use crate::pixel_ops::{add_pix, sub_pix, pix_as_u32};

    let (width, height) = img.dimensions() ;
    let mut sum_table = vec![[0; 4]; (width * height) as usize];

    sum_table[0] = pix_as_u32(img.get_pixel(0, 0).0);
    for x in 1..width {
        sum_table[x as usize] = add_pix(
            sum_table[x as usize - 1],
            pix_as_u32(img.get_pixel(x, 0).0)
        );
    }
    for y in 1..height {
        sum_table[(y * width) as usize] = add_pix(
            sum_table[((y - 1) * width) as usize],
            pix_as_u32(img.get_pixel(0, y).0)
        );
    }
    for y in 1..height {
        for x in 1..width {
            // sum[x,y] = sum[x-1,y] + sum[x,y-1] - sum[x-1,y-1] + img[x,y]
            sum_table[(x + y * width) as usize] = add_pix(
                sub_pix(
                    add_pix(
                        sum_table[(x - 1 + y * width) as usize],
                        sum_table[(x + (y - 1) * width) as usize]
                    ),
                    sum_table[(x - 1 + (y - 1) * width) as usize]
                ),
                pix_as_u32(img.get_pixel(x, y).0)
            );
        }
    }

    let mut buffer = image::ImageBuffer::new(width, height);

    for y in 0..height {
        let y_max = y.saturating_add(radius).min(height - 1);
        let y_min = y.saturating_sub(radius + 1);
        let y_len = y_max - y_min;

        for x in 0..width {
            let x_mas = x.saturating_add(radius).min(width - 1);
            let x_min = x.saturating_sub(radius + 1);

            let pix_max = sum_table[(x_mas + y_max * width) as usize];
            let pix_min = sum_table[(x_min + y_min * width) as usize];
            let pix_min_col = sum_table[(x_mas + y_min * width) as usize];
            let pix_min_row = sum_table[(x_min + y_max * width) as usize];
            let neighbours = (x_mas - x_min) * y_len;

            let sum = [
                ((pix_max[0] + pix_min[0] - pix_min_col[0] - pix_min_row[0]) / neighbours) as u8,
                ((pix_max[1] + pix_min[1] - pix_min_col[1] - pix_min_row[1]) / neighbours) as u8,
                ((pix_max[2] + pix_min[2] - pix_min_col[2] - pix_min_row[2]) / neighbours) as u8,
                ((pix_max[3] + pix_min[3] - pix_min_col[3] - pix_min_row[3]) / neighbours) as u8,
            ];

            buffer.put_pixel(x, y, image::Rgba(sum))
        }
    }

    buffer
}

pub fn erosion(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(img, radius, [u8::MAX; 4],
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
    compute_buffer(img, radius, [u8::MIN; 4],
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

    compute_buffer(img, radius, accumulator,
        |pix, vec| {
            let brightness = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
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
        let y_max = y.saturating_add(radius + 1).min(height - 1);
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
    use crate::file::get_new_image_file;
    use crate::filter::{
        flou_moyen, optimized_blur, erosion, dilatation, median,
        Buffer
    };

    const RADIUS: u32 = 120;
    const IMG: &str = "images/lena_1960.jpg";

    fn global_test(algo_name: &str, algo: fn(&Buffer, u32) -> Buffer) -> Result<(), Box<dyn Error>> {
        let fname = format!("_{}.", algo_name);
        let path = PathBuf::from(IMG);

        let dest = get_new_image_file(&path, &fname)?;
        let img = image::open(path)?.into_rgba8();

        let start = std::time::Instant::now();
        let buffer = algo(&img, RADIUS);
        let elapsed = start.elapsed().as_millis();
        println!("{}: {} ms", algo_name, elapsed);

        buffer.save(&dest)?;
        Ok(())
    }

    #[test]
    fn test_flou_moyen() -> Result<(), Box<dyn Error>> {
        global_test("flou_moyen", flou_moyen)
    }

    #[bench]
    fn bench_flou_moyen(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let img = image::open(path)?.into_rgba8();

        b.iter(|| flou_moyen(&img, RADIUS));
        Ok(())
    }

    #[test]
    fn test_flou_moyen_opt() -> Result<(), Box<dyn Error>> {
        global_test("optimized_blur", optimized_blur)
    }

    #[bench]
    fn bench_flou_moyen_opt(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let img = image::open(path)?.into_rgba8();

        b.iter(|| optimized_blur(&img, RADIUS));
        Ok(())
    }

    #[test]
    fn test_erosion() -> Result<(), Box<dyn Error>> {
        global_test("erosion", erosion)
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
        global_test("dilatation", dilatation)
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
        global_test("median", median)
    }

    #[bench]
    fn bench_median(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(IMG);
        let img = image::open(path)?.into_rgba8();

        b.iter(|| median(&img, RADIUS));
        Ok(())
    }
}
