use super::{Buffer, compute_buffer};

pub fn dilate(img: &Buffer, radius: u32) -> Buffer {
    let func = |pix: &[u8; 4], max: &mut [u8; 4]| {
        max[0] = if max[0] < pix[0] { pix[0] } else { max[0] };
        max[1] = if max[1] < pix[1] { pix[1] } else { max[1] };
        max[2] = if max[2] < pix[2] { pix[2] } else { max[2] };
        max[3] = if max[3] < pix[3] { pix[3] } else { max[3] };
    };

    compute_buffer(img, radius, [u8::MIN; 4],
        func, func, |max, _| max
    )
}
