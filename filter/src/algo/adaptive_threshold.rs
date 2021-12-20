use crate::pixel::Pixel;
use super::RgbaImage;

pub fn adaptive_threshold(img: &RgbaImage, radius: usize, f: i32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let (width, height) = (width as usize, height as usize);
    let mut sum_table = vec![Pixel::default(); width * height];

    sum_table[0] = Pixel::new(img.get_pixel(0, 0).0).into::<i32>();
    for x in 1..width {
        let next_pix = Pixel::new(img.get_pixel(x as u32, 0).0).into();
        sum_table[x] = sum_table[x - 1] + next_pix;
    }
    for y in 1..height {
        let next_pix = Pixel::new(img.get_pixel(0, y as u32).0).into();
        sum_table[y * width] = sum_table[(y - 1) * width] + next_pix;
    }
    for y in 1..height {
        for x in 1..width {
            let next_pix = Pixel::new(img.get_pixel(x as u32, y as u32).0).into();
            sum_table[x + y * width] = sum_table[x - 1 + y * width] + sum_table[x + (y - 1) * width] - sum_table[x - 1 + (y - 1) * width] + next_pix;
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
            let pix_min = if overflow_x || overflow_y { Pixel::default() } else { sum_table[x_min + y_min * width] };
            let pix_min_col = if overflow_y { Pixel::default() } else { sum_table[x_max + y_min * width] };
            let pix_min_row = if overflow_x { Pixel::default() } else { sum_table[x_min + y_max * width] };
            let neighbours = ((x_max - x_min + overflow_x as usize) * y_len) as i32;

            let pix = img.get_pixel(x as u32, y as u32).0;
            let gray_pix = Pixel::new(pix).as_gray();

            let threshold = ((pix_max + pix_min - pix_min_col - pix_min_row) / neighbours).as_gray();
            let threshold = (threshold + f).clamp(0, 255) as u8;

            let new_pix = image::Rgba(if gray_pix > threshold { pix } else { [0, 0, 0, 255] });
            buffer.put_pixel(x as u32, y as u32, new_pix);
        }
    }

    buffer
}
