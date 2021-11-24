use std::{error::Error, path::PathBuf};
use filter::{
    flou_moyen, optimized_blur, erode, dilate, median_blur,
    Buffer
};

const RADIUS: u32 = 2;
const IMG: &str = "../static/images/lena";
const EXT: &str = "jpg";

fn global_test(algo_name: &str, algo: fn(&Buffer, u32) -> Buffer) -> Result<(), Box<dyn Error>> {
    let source = PathBuf::from(format!("{}.{}", IMG, EXT));
    let img = image::open(&source)?.into_rgba8();

    let start = std::time::Instant::now();
    let buffer = algo(&img, RADIUS);
    let elapsed = start.elapsed().as_millis();
    println!("{}: {} ms", algo_name, elapsed);

    let dest = PathBuf::from(format!("{}_{}.{}", IMG, algo_name, EXT));
    buffer.save(&dest)?;
    Ok(())
}

#[test]
fn test_flou_moyen() -> Result<(), Box<dyn Error>> {
    global_test("flou_moyen", flou_moyen)
}

#[test]
fn test_flou_moyen_opt() -> Result<(), Box<dyn Error>> {
    global_test("optimized_blur", optimized_blur)
}

#[test]
fn test_erosion() -> Result<(), Box<dyn Error>> {
    global_test("erode", erode)
}

#[test]
fn test_dilatation() -> Result<(), Box<dyn Error>> {
    global_test("dilate", dilate)
}

#[test]
fn test_median() -> Result<(), Box<dyn Error>> {
    global_test("median_blur", median_blur)
}
