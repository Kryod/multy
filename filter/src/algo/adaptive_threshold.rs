use super::RgbaImage;
use crate::pixel;

pub fn adaptive_threshold(img: &RgbaImage, radius: usize, f: i32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let (width, height) = (width as usize, height as usize);
    let mut sum_table = vec![[0; 4]; width * height];

    sum_table[0] = pixel::as_u32(img.get_pixel(0, 0).0);
    for x in 1..width {
        sum_table[x] = pixel::add(
            sum_table[x - 1],
            pixel::as_u32(img.get_pixel(x as u32, 0).0)
        );
    }
    for y in 1..height {
        sum_table[y * width] = pixel::add(
            sum_table[(y - 1) * width],
            pixel::as_u32(img.get_pixel(0, y as u32).0)
        );
    }
    for y in 1..height {
        for x in 1..width {
            // sum[x,y] = sum[x-1,y] + sum[x,y-1] - sum[x-1,y-1] + img[x,y]
            sum_table[x + y * width] = pixel::add(
                pixel::sub(
                    pixel::add(
                        sum_table[x - 1 + y * width],
                        sum_table[x + (y - 1) * width]
                    ),
                    sum_table[x - 1 + (y - 1) * width]
                ),
                pixel::as_u32(img.get_pixel(x as u32, y as u32).0)
            );
        }
    }

    let mut buffer = RgbaImage::new(width as u32, height as u32);

    for y in 0..height {
        let y_max = y.saturating_add(radius).min(height - 1);
        let (y_min, overflow_y) = match y.overflowing_sub(radius + 1) {
            (_, true) => (usize::MIN, true),
            sub => sub,
        };
        let y_len = y_max - y_min + overflow_y as usize;

        for x in 0..width {
            let x_max = x.saturating_add(radius).min(width - 1);
            let (x_min, overflow_x) = match x.overflowing_sub(radius + 1) {
                (_, true) => (usize::MIN, true),
                sub => sub,
            };

            let pix_max = sum_table[x_max + y_max * width];
            let pix_min = if overflow_x || overflow_y { [0; 4] } else { sum_table[x_min + y_min * width] };
            let pix_min_col = if overflow_y { [0; 4] } else { sum_table[x_max + y_min * width] };
            let pix_min_row = if overflow_x { [0; 4] } else { sum_table[x_min + y_max * width] };
            let neighbours = ((x_max - x_min + overflow_x as usize) * y_len) as u32;

            let pix = img.get_pixel(x as u32, y as u32).0;
            let gray_pix = pixel::as_gray(pix);

            let threshold = [
                ((pix_max[0] + pix_min[0] - pix_min_col[0] - pix_min_row[0]) / neighbours) as u8,
                ((pix_max[1] + pix_min[1] - pix_min_col[1] - pix_min_row[1]) / neighbours) as u8,
                ((pix_max[2] + pix_min[2] - pix_min_col[2] - pix_min_row[2]) / neighbours) as u8,
                0
            ];
            let threshold = (pixel::as_gray(threshold) as i32 + f).clamp(0, 255) as u8;
            let new_pix = image::Rgba(if gray_pix > threshold { pix } else { [0, 0, 0, 255] });

            buffer.put_pixel(x as u32, y as u32, new_pix);
        }
    }

    buffer
}
