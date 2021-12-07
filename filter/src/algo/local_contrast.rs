use super::Buffer;
use crate::pixel;

pub fn local_contrast(img: &Buffer, radius: u32, factor: f32) -> Buffer {
    let (width, height) = img.dimensions();
    let mut buffer = Buffer::new(width, height);

    for y in 0..height {
        let y_max = y.saturating_add(radius + 1).min(height);
        let y_min = y.saturating_sub(radius);
        let y_len = y_max - y_min;

        for x in 0..width {
            let x_max = x.saturating_add(radius + 1).min(width);
            let x_min = x.saturating_sub(radius);

            let mut accumulator = [0; 4];
            for neighbour_y in y_min..y_max {
                for neighbour_x in x_min..x_max {
                    let pix = img.get_pixel(neighbour_x, neighbour_y).0;

                    accumulator = pixel::add(
                        accumulator,
                        pixel::as_u32(pix)
                    );
                }
            }

            let pix = img.get_pixel(x, y).0;
            let avg = pixel::div(accumulator, (x_max - x_min) * y_len);

            let contrast = [
                ((pix[0] as i32 - avg[0] as i32) as f32 * factor) as i32,
                ((pix[1] as i32 - avg[1] as i32) as f32 * factor) as i32,
                ((pix[2] as i32 - avg[2] as i32) as f32 * factor) as i32,
                ((pix[3] as i32 - avg[3] as i32) as f32 * factor) as i32,
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
