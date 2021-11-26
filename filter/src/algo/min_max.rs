use super::Buffer;

pub fn min_max(img: &Buffer, radius: u32) -> Buffer {
    let (width, height) = img.dimensions();
    let mut buffer = Buffer::new(width, height);

    for y in 0..height {
        let y_max = y.saturating_add(radius + 1).min(height);
        let y_min = y.saturating_sub(radius);

        for x in 0..width {
            let x_max = x.saturating_add(radius + 1).min(width);
            let x_min = x.saturating_sub(radius);
            let mut min = [u8::MAX; 4];
            let mut max = [u8::MIN; 4];

            for neighbour_y in y_min..y_max {
                for neighbour_x in x_min..x_max {
                    if (neighbour_x == x) && (neighbour_y == y) {
                        continue;
                    }

                    let pix = img.get_pixel(neighbour_x, neighbour_y).0;

                    min[0] = if min[0] < pix[0] { min[0] } else { pix[0] };
                    min[1] = if min[1] < pix[1] { min[1] } else { pix[1] };
                    min[2] = if min[2] < pix[2] { min[2] } else { pix[2] };
                    min[3] = if min[3] < pix[3] { min[3] } else { pix[3] };

                    max[0] = if max[0] < pix[0] { pix[0] } else { max[0] };
                    max[1] = if max[1] < pix[1] { pix[1] } else { max[1] };
                    max[2] = if max[2] < pix[2] { pix[2] } else { max[2] };
                    max[3] = if max[3] < pix[3] { pix[3] } else { max[3] };
                }
            }

            let pix = img.get_pixel(x, y).0;
            let min_max = [
                pix[0].clamp(min[0], max[0]),
                pix[1].clamp(min[1], max[1]),
                pix[2].clamp(min[2], max[2]),
                pix[3].clamp(min[3], max[3]),
            ];
            buffer.put_pixel(x, y, image::Rgba(min_max));
        }
    }

    buffer
}
