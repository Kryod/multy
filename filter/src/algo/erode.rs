use super::{Buffer, compute_buffer};

pub fn erode(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(img, radius, [u8::MAX; 4],
        |pix, min| {
            if min[0] > pix[0] { min[0] = pix[0] }
            if min[1] > pix[1] { min[1] = pix[1] }
            if min[2] > pix[2] { min[2] = pix[2] }
            if min[3] > pix[3] { min[3] = pix[3] }
        },
        |col, min| {
            if min[0] > col[0] { min[0] = col[0] }
            if min[1] > col[1] { min[1] = col[1] }
            if min[2] > col[2] { min[2] = col[2] }
            if min[3] > col[3] { min[3] = col[3] }
        },
        |min, _| min
    )
}
