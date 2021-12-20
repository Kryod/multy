use super::RgbaImage;
use crate::pixel;

pub fn median_blur(img: &RgbaImage, radius: u32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let capacity = (radius * 2 + 1).pow(2) as usize;
    let mut container = Vec::with_capacity(capacity);
    let mut buffer = RgbaImage::new(width, height);

    for y in 0..height {
        let y_max = y.saturating_add(radius + 1).min(height);
        let y_min = y.saturating_sub(radius);

        for x in 0..width {
            let x_max = x.saturating_add(radius + 1).min(width);
            let x_min = x.saturating_sub(radius);
            container.clear();

            for neighbour_y in y_min..y_max {
                for neighbour_x in x_min..x_max {
                    let pix = img.get_pixel(neighbour_x, neighbour_y).0;
                    let br = pixel::as_gray(pix);

                    container.push((br, pix));
                }
            }

            container.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
            let median = container[container.len() / 2].1;
            buffer.put_pixel(x, y, image::Rgba(median));
        }
    }

    buffer
}
