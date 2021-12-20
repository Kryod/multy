use super::{RgbaImage, compute_buffer};
use crate::pixel;

pub fn erode(img: &RgbaImage, radius: u32) -> RgbaImage {
    compute_buffer(img, radius, [u8::MAX; 4],
        pixel::min, pixel::min, |min, _| min
    )
}
