use std::{error::Error, path::PathBuf};
use multy::file::get_new_image_file;
use multy::filter::{
    flou_moyen, optimized_blur, erosion, dilatation, median,
    Buffer
};

const RADIUS: u32 = 2;
const IMG: &str = "static/images/lena.jpg";

fn global_test(algo_name: &str, algo: fn(&Buffer, u32) -> Buffer) -> Result<(), Box<dyn Error>> {
    let fname = format!("_{}.", algo_name);
    let path = PathBuf::from(IMG);

    let dest = get_new_image_file(&path, &fname)?;
    let img = image::open(path)?.into_rgba8();

    let start = std::time::Instant::now();
    let buffer = algo(&img, RADIUS);
    let elapsed = start.elapsed().as_millis();
    println!("{}: {} ms", algo_name, elapsed);

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
    global_test("erosion", erosion)
}

#[test]
fn test_dilatation() -> Result<(), Box<dyn Error>> {
    global_test("dilatation", dilatation)
}

#[test]
fn test_median() -> Result<(), Box<dyn Error>> {
    global_test("median", median)
}
