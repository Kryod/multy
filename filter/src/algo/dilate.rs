use super::{Buffer, compute_buffer};
use crate::pixel;

pub fn dilate(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(img, radius, [u8::MIN; 4],
        pixel::max, pixel::max, |max, _| max
    )
}
