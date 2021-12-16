use super::Buffer;
use crate::pixel;

pub fn adaptive_threshold(img: &Buffer, radius: u32, f: i32) -> Buffer {
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
            let pix_min_col = if overflow_y { [0; 4] } else { sum_table[(x_max + y_min * width) as usize] };
            let pix_min_row = if overflow_x { [0; 4] } else { sum_table[(x_min + y_max * width) as usize] };
            let neighbours = (x_max - x_min + overflow_x as u32) * y_len;

            let pix = img.get_pixel(x, y).0;
            let gray_pix = pixel::as_gray(pix);

            let threshold = [
                ((pix_max[0] + pix_min[0] - pix_min_col[0] - pix_min_row[0]) / neighbours) as u8,
                ((pix_max[1] + pix_min[1] - pix_min_col[1] - pix_min_row[1]) / neighbours) as u8,
                ((pix_max[2] + pix_min[2] - pix_min_col[2] - pix_min_row[2]) / neighbours) as u8,
                0
            ];
            let threshold = (pixel::as_gray(threshold) as i32 + f).clamp(0, 255) as u8;

            let new_pix = image::Rgba(
                if gray_pix > threshold { pix } else { [0, 0, 0, 255] }
                // if gray_pix > threshold { [255; 4] } else { [0, 0, 0, 255] }
            );

            buffer.put_pixel(x, y, new_pix);
        }
    }

    buffer
}

/*
def adaptive_threshold(im, radius, f):
    copy = im.copy()
    data1 = im.load()
    data2 = copy.load()
    size = copy.size

    for y in range(size[1]):
        for x in range(size[0]):
            ecart = 0
            nb = 0

            for y_ in range(y - radius, y + radius + 1):
                if y_ < 0 or y_ >= size[1]:
                    continue

                for x_ in range(x - radius, x + radius + 1):
                    if x_ < 0 or x_ >= size[0]:
                        continue

                    ecart += data1[x_, y_]
                    nb += 1

            data2[x, y] = 255 if data1[x, y] > int(ecart / nb) + f else 0

    return copy
*/
