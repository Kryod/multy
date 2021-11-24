use super::{Buffer, compute_buffer};

pub fn median_blur(img: &Buffer, radius: u32) -> Buffer {
    let capacity = (radius * 2 + 1).pow(2) as usize;
    let accumulator = Vec::with_capacity(capacity);

    compute_buffer(img, radius, accumulator,
        |pix, vec| {
            let brightness = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
            vec.push((brightness, *pix));
        },
        |col, vec| {
            vec.extend(col);
        },
        |mut vec, neighbours| {
            vec.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
            vec[(neighbours / 2) as usize].1
        }
    )
}
