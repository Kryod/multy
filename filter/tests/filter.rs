use std::{error::Error, path::PathBuf};
use filter::{self, Buffer};

const RADIUS: u32 = 1;
const IMG: &str = "../static/images/lena_really_noise";
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
fn blur() -> Result<(), Box<dyn Error>> {
    global_test("blur", filter::blur)
}

#[test]
fn erode() -> Result<(), Box<dyn Error>> {
    global_test("erode", filter::erode)
}

#[test]
fn dilate() -> Result<(), Box<dyn Error>> {
    global_test("dilate", filter::dilate)
}

#[test]
fn median_blur() -> Result<(), Box<dyn Error>> {
    global_test("median_blur", filter::median_blur)
}

#[test]
fn min_max() -> Result<(), Box<dyn Error>> {
    global_test("min_max", filter::min_max)
}
