use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

pub fn flou_moyen(path: PathBuf, radius: u32) -> PathBuf {
    let dest = get_new_image_file(&path, "_flou_moyen.");
    let img = image::open(path).unwrap();

    compute_and_save_buffer(img, radius, &dest, [0; 4],
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
    );

    dest
}

pub fn erosion(path: PathBuf) -> PathBuf {
    let dest = get_new_image_file(&path, "_erosion.");
    let img = image::open(path).unwrap();
    let radius = 2;

    compute_and_save_buffer(img, radius, &dest, [u32::MAX; 4],
        |pix, min| {
            if min[0] > pix[0] as u32 { min[0] = pix[0] as u32 }
            if min[1] > pix[1] as u32 { min[1] = pix[1] as u32 }
            if min[2] > pix[2] as u32 { min[2] = pix[2] as u32 }
            if min[3] > pix[3] as u32 { min[3] = pix[3] as u32 }
        },
        |col, min| {
            if min[0] > col[0] { min[0] = col[0] }
            if min[1] > col[1] { min[1] = col[1] }
            if min[2] > col[2] { min[2] = col[2] }
            if min[3] > col[3] { min[3] = col[3] }
        },
        |min, _| [
            min[0] as u8,
            min[1] as u8,
            min[2] as u8,
            min[3] as u8,
        ]
    );

    dest
}

fn get_new_image_file(path: &PathBuf, file_name_add: &str) -> PathBuf {
    let file_stem = path.file_stem().unwrap();
    let extension = path.extension().unwrap();

    // prevent string realloc
    let mut new_path =
        String::with_capacity(file_stem.len() + file_name_add.len() + extension.len());

    new_path.push_str(file_stem.to_str().unwrap());
    new_path.push_str(file_name_add);
    new_path.push_str(extension.to_str().unwrap());

    let base_path = "images";
    let mut to_save = PathBuf::with_capacity(base_path.len() + new_path.len());

    to_save.push(base_path);
    to_save.push(new_path);

    to_save
}

fn compute_and_save_buffer(
    img: DynamicImage,
    radius: u32,
    dest: &PathBuf,
    accumulator: [u32; 4],
    reduce: fn(&[u8; 4], &mut [u32; 4]),
    concat: fn(&[u32; 4], &mut [u32; 4]),
    average: fn([u32; 4], u32) -> [u8; 4],
) {
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
            let mut acc = accumulator;

            for neighbour_y in y_min..y_max {
                let pix = &img.get_pixel(neighbour_x, neighbour_y).0;
                reduce(pix, &mut acc);
            }

            partial_blur.push_back(acc);
        }

        // compute every Pixels
        for x in 0..width {
            let mut acc = accumulator;
            let neighbours = (y_max - y_min) * partial_blur.len() as u32;

            for col in &partial_blur {
                concat(col, &mut acc);
            }

            let avg = average(acc, neighbours);
            buffer.put_pixel(x, y, image::Rgba(avg));

            // compute next partial blur row
            let x_target = x.saturating_add(radius + 1);
            if x_target < width {
                let mut acc = accumulator;

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

    buffer.save(dest).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_flou_moyen() {
        let start = std::time::Instant::now();
        super::flou_moyen(PathBuf::from("images/lena.jpg"), 2);

        let elapsed = start.elapsed().as_millis();
        println!("flou_moyen: {} ms", elapsed);
    }

    #[test]
    fn test_erosion() {
        let start = std::time::Instant::now();
        super::erosion(PathBuf::from("images/lena.jpg"));

        let elapsed = start.elapsed().as_millis();
        println!("erosion: {} ms", elapsed);
    }
}
