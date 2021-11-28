use super::{Buffer, compute_buffer};

pub fn dilate(img: &Buffer, radius: u32) -> Buffer {
    use crate::pixel_ops::max_pix;

    compute_buffer(img, radius, [u8::MIN; 4],
        max_pix, max_pix, |max, _| max
    )
}
