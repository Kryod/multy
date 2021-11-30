use super::{Buffer, compute_buffer};
use crate::pixel;

pub fn erode(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(img, radius, [u8::MAX; 4],
        pixel::min, pixel::min, |min, _| min
    )
}
