use super::{Buffer, compute_buffer};
use crate::pixel_ops;

pub fn erode(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(img, radius, [u8::MAX; 4],
        pixel_ops::min_pix, pixel_ops::min_pix, |min, _| min
    )
}
