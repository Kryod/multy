use super::{Buffer, compute_buffer};

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

    let (width, height) = img.dimensions();
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

    let mut buffer = Buffer::new(width, height);

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
