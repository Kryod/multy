use super::Buffer;

#[derive(Debug)]
pub enum CompareError {
    MismatchSize,
}

const SAME: [u8; 4] = [0, 0, 0, 255];
const DIFF: [u8; 4] = [190, 0, 0, 255];

pub fn compare(lhs: &Buffer, rhs: &Buffer) -> Result<Buffer, CompareError> {
    let dim = lhs.dimensions();
    if dim != rhs.dimensions() {
        return Err(CompareError::MismatchSize);
    }

    let mut buffer = Buffer::new(dim.0, dim.1);
    buffer.pixels_mut().zip(lhs.pixels().zip(rhs.pixels())).for_each(|(buffer, (lhs, rhs))|
        *buffer = if lhs == rhs {
            image::Rgba(SAME)
        } else {
            image::Rgba(DIFF)
        }
    );

    Ok(buffer)
}

impl std::fmt::Display for CompareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The two images doesn't have the same size.")
    }
}

impl std::error::Error for CompareError {}
