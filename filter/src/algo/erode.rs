use super::{Buffer, compute_buffer};

pub fn erode(img: &Buffer, radius: u32) -> Buffer {
    let func = |pix: &[u8; 4], min: &mut [u8; 4]| {
        min[0] = if min[0] < pix[0] { min[0] } else { pix[0] };
        min[1] = if min[1] < pix[1] { min[1] } else { pix[1] };
        min[2] = if min[2] < pix[2] { min[2] } else { pix[2] };
        min[3] = if min[3] < pix[3] { min[3] } else { pix[3] };
    };

    compute_buffer(img, radius, [u8::MAX; 4],
        func, func, |min, _| min
    )
}
