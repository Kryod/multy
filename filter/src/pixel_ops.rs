#[inline]
pub fn pix_as_u32(pix: [u8; 4]) -> [u32; 4] {
    [
        pix[0] as u32,
        pix[1] as u32,
        pix[2] as u32,
        pix[3] as u32,
    ]
}

#[inline]
pub fn add_pix(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
    [
        lhs[0] + rhs[0],
        lhs[1] + rhs[1],
        lhs[2] + rhs[2],
        lhs[3] + rhs[3],
    ]
}

#[inline]
pub fn sub_pix(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
    [
        lhs[0] - rhs[0],
        lhs[1] - rhs[1],
        lhs[2] - rhs[2],
        lhs[3] - rhs[3],
    ]
}
