use super::{Buffer, compute_buffer};
use crate::pixel_ops;

pub fn dilate(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(img, radius, [u8::MIN; 4],
        pixel_ops::max_pix, pixel_ops::max_pix, |max, _| max
    )
}
