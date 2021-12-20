use super::{RgbaImage, compute_buffer};
use crate::pixel;

pub fn dilate(img: &RgbaImage, radius: u32) -> RgbaImage {
    compute_buffer(img, radius, [u8::MIN; 4],
        pixel::max, pixel::max, |max, _| max
    )
}
