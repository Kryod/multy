use super::{Buffer, compute_buffer};

pub fn dilate(img: &Buffer, radius: u32) -> Buffer {
    compute_buffer(img, radius, [u8::MIN; 4],
        |pix, max| {
            if max[0] < pix[0] { max[0] = pix[0] }
            if max[1] < pix[1] { max[1] = pix[1] }
            if max[2] < pix[2] { max[2] = pix[2] }
            if max[3] < pix[3] { max[3] = pix[3] }
        },
        |col, max| {
            if max[0] < col[0] { max[0] = col[0] }
            if max[1] < col[1] { max[1] = col[1] }
            if max[2] < col[2] { max[2] = col[2] }
            if max[3] < col[3] { max[3] = col[3] }
        },
        |max, _| max
    )
}
