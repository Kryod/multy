#[derive(Debug, Default, Clone, Copy)]
pub struct Pixel<T>(pub [T; 4]);

impl<T> Pixel<T> {
    pub fn new(data: [T; 4]) -> Self {
        Pixel(data)
    }

    pub fn into<U>(self) -> Pixel<U> where U:
        TryFrom<T> + Default
    {
        let [a, b, c, d] = self.0;

        Pixel([
            a.try_into().unwrap_or_default(),
            b.try_into().unwrap_or_default(),
            c.try_into().unwrap_or_default(),
            d.try_into().unwrap_or_default(),
        ])
    }

    pub fn clamp(self, min: T, max: T) -> Self where T:
        Ord + Copy
    {
        let [a, b, c, d] = self.0;

        Pixel([
            a.clamp(min, max),
            b.clamp(min, max),
            c.clamp(min, max),
            d.clamp(min, max),
        ])
    }
}

impl<T> Pixel<T> where T:
    std::ops::Div<Output = T> +
    std::ops::Mul<Output = T> +
    std::ops::Add<Output = T> +
    From<u8>
{
    pub fn as_gray(self) -> T {
        let [r, g, b, _] = self.0;

        r / 10.into() * 3.into() + // 0.299
        g / 10.into() * 6.into() + // 0.587
        b / 10.into()              // 0.114
    }
}

impl<T> std::ops::Add for Pixel<T> where T:
    std::ops::Add<Output = T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let [a0, b0, c0, d0] = self.0;
        let [a1, b1, c1, d1] = rhs.0;

        Pixel([
            a0 + a1,
            b0 + b1,
            c0 + c1,
            d0 + d1,
        ])
    }
}

impl<T> std::ops::Sub for Pixel<T> where T:
    std::ops::Sub<Output = T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let [a0, b0, c0, d0] = self.0;
        let [a1, b1, c1, d1] = rhs.0;

        Pixel([
            a0 - a1,
            b0 - b1,
            c0 - c1,
            d0 - d1,
        ])
    }
}

impl<T> std::ops::Div<T> for Pixel<T> where T:
    std::ops::Div<Output = T> + Copy
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let [a, b, c, d] = self.0;
        Pixel([a / rhs, b / rhs, c / rhs, d / rhs])
    }
}

impl<T> std::ops::Mul<T> for Pixel<T> where T:
    std::ops::Mul<Output = T> + Copy
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let [a, b, c, d] = self.0;
        Pixel([a * rhs, b * rhs, c * rhs, d * rhs])
    }
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
