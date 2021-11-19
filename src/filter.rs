use std::path::PathBuf;
use image::GenericImageView;

pub fn flou_moyen(path: PathBuf, radius: u32) -> PathBuf {
    let file_stem = path.file_stem().unwrap();
    let flou_moyen = "_flou_moyen.";
    let extension = path.extension().unwrap();

    // prevent string realloc
    let mut new_path = String::with_capacity(
        file_stem.len() + flou_moyen.len() + extension.len()
    );

    new_path.push_str(file_stem.to_str().unwrap());
    new_path.push_str(flou_moyen);
    new_path.push_str(extension.to_str().unwrap());

    let base_path = "images";
    let mut to_save = PathBuf::with_capacity(
        base_path.len() + new_path.len()
    );

    to_save.push(base_path);
    to_save.push(new_path);

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
    to_save
}

#[test]
fn test_flou_moyen() {
    let start = std::time::Instant::now();
    flou_moyen(PathBuf::from("images/lena.jpg"), 2);

    let elapsed = start.elapsed().as_millis();
    println!("flou_moyen: {} ms", elapsed);
}
