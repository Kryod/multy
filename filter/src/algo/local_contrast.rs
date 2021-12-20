use crate::pixel::Pixel;
use super::RgbaImage;

pub fn local_contrast(img: &RgbaImage, radius: u32, factor: i32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let (uwidth, uheight) = (width as usize, height as usize);
    let mut sum_table = vec![Pixel::default(); (width * height) as usize];

    sum_table[0] = Pixel::new(img.get_pixel(0, 0).0).into::<u32>();
    for x in 1..uwidth {
        let next_pix = Pixel::new(img.get_pixel(x as u32, 0).0).into();
        sum_table[x] = sum_table[x - 1] + next_pix;
    }
    for y in 1..uheight {
        let next_pix = Pixel::new(img.get_pixel(0, y as u32).0).into();
        sum_table[y * uwidth] = sum_table[(y - 1) * uwidth] + next_pix;
    }
    for y in 1..uheight {
        for x in 1..uwidth {
            let next_pix = Pixel::new(img.get_pixel(x as u32, y as u32).0).into();
            sum_table[x + y * uwidth] = sum_table[x - 1 + y * uwidth] + sum_table[x + (y - 1) * uwidth] - sum_table[x - 1 + (y - 1) * uwidth] + next_pix;
        }
    }

    let mut buffer = RgbaImage::new(width, height);

    for y in 0..height {
        let y_max = y.saturating_add(radius).min(height - 1);
        let (y_min, overflow_y) = match y.overflowing_sub(radius + 1) {
            (_, true) => (u32::MIN, true),
            sub => sub,
        };
        let y_len = y_max - y_min + overflow_y as u32;

        for x in 0..width {
            let x_max = x.saturating_add(radius).min(width - 1);
            let (x_min, overflow_x) = match x.overflowing_sub(radius + 1) {
                (_, true) => (u32::MIN, true),
                sub => sub,
            };

            let pix_max = sum_table[(x_max + y_max * width) as usize];
            let pix_min = if overflow_x || overflow_y { Pixel::default() } else { sum_table[(x_min + y_min * width) as usize] };
            let pix_min_col = if overflow_y { Pixel::default() } else { sum_table[(x_max + y_min * width) as usize]};
            let pix_min_row = if overflow_x { Pixel::default() } else { sum_table[(x_min + y_max * width) as usize]};
            let neighbours = (x_max - x_min + overflow_x as u32) * y_len;

            let avg = ((pix_max + pix_min - pix_min_col - pix_min_row) / neighbours).into();
            let pix = Pixel::new(img.get_pixel(x, y).0).into();
            let contrast = (pix - avg) * factor;

            buffer.put_pixel(x, y, image::Rgba((pix + contrast).clamp(0, 255).into().0));
        }
    }

    buffer
}
