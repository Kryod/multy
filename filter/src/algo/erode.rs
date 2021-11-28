use super::{Buffer, compute_buffer};

pub fn erode(img: &Buffer, radius: u32) -> Buffer {
    use crate::pixel_ops::min_pix;

    compute_buffer(img, radius, [u8::MAX; 4],
        min_pix, min_pix, |min, _| min
    )
}
