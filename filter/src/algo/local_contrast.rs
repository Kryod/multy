use super::Buffer;
use crate::pixel;

pub fn local_contrast(img: &Buffer, radius: u32, factor: i32) -> Buffer {
    let (width, height) = img.dimensions();
    let mut sum_table = vec![[0; 4]; (width * height) as usize];

    sum_table[0] = pixel::as_u32(img.get_pixel(0, 0).0);
    for x in 1..width {
        sum_table[x as usize] = pixel::add(
            sum_table[x as usize - 1],
            pixel::as_u32(img.get_pixel(x, 0).0)
        );
    }
    for y in 1..height {
        sum_table[(y * width) as usize] = pixel::add(
            sum_table[((y - 1) * width) as usize],
            pixel::as_u32(img.get_pixel(0, y).0)
        );
    }
    for y in 1..height {
        for x in 1..width {
            // sum[x,y] = sum[x-1,y] + sum[x,y-1] - sum[x-1,y-1] + img[x,y]
            sum_table[(x + y * width) as usize] = pixel::add(
                pixel::sub(
                    pixel::add(
                        sum_table[(x - 1 + y * width) as usize],
                        sum_table[(x + (y - 1) * width) as usize]
                    ),
                    sum_table[(x - 1 + (y - 1) * width) as usize]
                ),
                pixel::as_u32(img.get_pixel(x, y).0)
            );
        }
    }

    let mut buffer = Buffer::new(width, height);

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
            let pix_min = if overflow_x || overflow_y { [0; 4] } else { sum_table[(x_min + y_min * width) as usize] };
            let pix_min_col = if overflow_y { [0; 4] } else { sum_table[(x_max + y_min * width) as usize]};
            let pix_min_row = if overflow_x { [0; 4] } else { sum_table[(x_min + y_max * width) as usize]};
            let neighbours = (x_max - x_min + overflow_x as u32) * y_len;

            let pix = img.get_pixel(x, y).0;
            let avg = [
                ((pix_max[0] + pix_min[0] - pix_min_col[0] - pix_min_row[0]) / neighbours) as i32,
                ((pix_max[1] + pix_min[1] - pix_min_col[1] - pix_min_row[1]) / neighbours) as i32,
                ((pix_max[2] + pix_min[2] - pix_min_col[2] - pix_min_row[2]) / neighbours) as i32,
                ((pix_max[3] + pix_min[3] - pix_min_col[3] - pix_min_row[3]) / neighbours) as i32,
            ];

            let contrast = [
                (pix[0] as i32 - avg[0])* factor,
                (pix[1] as i32 - avg[1])* factor,
                (pix[2] as i32 - avg[2])* factor,
                (pix[3] as i32 - avg[3])* factor,
            ];

            buffer.put_pixel(x, y, image::Rgba([
                (pix[0] as i32 + contrast[0]).clamp(0, 255) as u8,
                (pix[1] as i32 + contrast[1]).clamp(0, 255) as u8,
                (pix[2] as i32 + contrast[2]).clamp(0, 255) as u8,
                (pix[3] as i32 + contrast[3]).clamp(0, 255) as u8,
            ]));
        }
    }

    buffer
}
