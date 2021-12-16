mod pixel;
mod algo;

pub use algo::{
    Buffer, Algorithms, run_algo, compare_images,
    compare::compare,

    adaptive_threshold::adaptive_threshold,
    local_contrast::local_contrast,
    median_blur::median_blur,
    min_max::min_max,
    dilate::dilate,
    erode::erode,
    blur::blur,
};

// reexport for imgerror
pub use image::ImageError;
