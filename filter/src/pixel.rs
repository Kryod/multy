#[inline]
pub fn as_u32(pix: [u8; 4]) -> [u32; 4] {
    [
        pix[0] as u32,
        pix[1] as u32,
        pix[2] as u32,
        pix[3] as u32,
    ]
}

#[inline]
pub fn as_gray(pix: [u8; 4]) -> u8 {
    pix[0] / 10 * 3 + // 0.299
    pix[1] / 10 * 6 + // 0.587
    pix[2] / 10       // 0.114
}

#[inline]
pub fn add(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
    [
        lhs[0] + rhs[0],
        lhs[1] + rhs[1],
        lhs[2] + rhs[2],
        lhs[3] + rhs[3],
    ]
}

#[inline]
pub fn sub(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
    [
        lhs[0] - rhs[0],
        lhs[1] - rhs[1],
        lhs[2] - rhs[2],
        lhs[3] - rhs[3],
    ]
}

#[inline]
pub fn div(pix: [u32; 4], rhs: u32) -> [u32; 4] {
    [
        pix[0] / rhs,
        pix[1] / rhs,
        pix[2] / rhs,
        pix[3] / rhs,
    ]
}

#[inline]
pub fn min(pix: &[u8; 4], min: &mut [u8; 4]) {
    min[0] = if min[0] < pix[0] { min[0] } else { pix[0] };
    min[1] = if min[1] < pix[1] { min[1] } else { pix[1] };
    min[2] = if min[2] < pix[2] { min[2] } else { pix[2] };
    min[3] = if min[3] < pix[3] { min[3] } else { pix[3] };
}

#[inline]
pub fn max(pix: &[u8; 4], max: &mut [u8; 4]) {
    max[0] = if max[0] < pix[0] { pix[0] } else { max[0] };
    max[1] = if max[1] < pix[1] { pix[1] } else { max[1] };
    max[2] = if max[2] < pix[2] { pix[2] } else { max[2] };
    max[3] = if max[3] < pix[3] { pix[3] } else { max[3] };
}
