mod pixel_ops;
mod algo;

pub use algo::{
    Buffer, Algorithms, run_algo,
    median_blur::median_blur,
    dilate::dilate,
    erode::erode,
    blur::blur,
};

// reexport for imgerror
pub use image::ImageError;
