mod pixel_ops;
mod algo;

pub use algo::{
    Buffer, Algorithms, run_algo,
    blur::{flou_moyen, optimized_blur},
    median_blur::median_blur,
    dilate::dilate,
    erode::erode,
};

// reexport for imgerror
pub use image::ImageError;
