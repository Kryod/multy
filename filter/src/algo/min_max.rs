use super::RgbaImage;
use crate::pixel;

pub fn min_max(img: &RgbaImage, radius: u32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut buffer = RgbaImage::new(width, height);

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
                    pixel::min(&pix, &mut min);
                    pixel::max(&pix, &mut max);
                }
            }

            let pix = img.get_pixel(x, y).0;
            let min_max = [
                if pix[0] < min[0] { min[0] } else if pix[0] < max[0] { pix[0] } else { max[0] },
                if pix[1] < min[1] { min[1] } else if pix[1] < max[1] { pix[1] } else { max[1] },
                if pix[2] < min[2] { min[2] } else if pix[2] < max[2] { pix[2] } else { max[2] },
                if pix[3] < min[3] { min[3] } else if pix[3] < max[3] { pix[3] } else { max[3] },
            ];

            buffer.put_pixel(x, y, image::Rgba(min_max));
        }
    }

    buffer
}
