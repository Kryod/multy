#![feature(test)]

extern crate test;
use test::Bencher;

use std::{error::Error, path::PathBuf};


const RADIUS: u32 = 2;
const FACTOR: i32 = 5;
const IMG: &str = "../static/images/lena.jpg";

#[bench]
fn blur(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| filter::blur(&img, RADIUS));
    Ok(())
}

#[bench]
fn dilate(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| filter::dilate(&img, RADIUS));
    Ok(())
}

#[bench]
fn erode(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| filter::erode(&img, RADIUS));
    Ok(())
}

#[bench]
fn local_contrast(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| filter::local_contrast(&img, RADIUS, FACTOR));
    Ok(())
}

#[bench]
fn median_blur(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| filter::median_blur(&img, RADIUS));
    Ok(())
}

#[bench]
fn min_max(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| filter::min_max(&img, RADIUS));
    Ok(())
}
