#![feature(test)]

extern crate test;
use test::Bencher;

use std::{error::Error, path::PathBuf};
use multy::filter::{
    flou_moyen, optimized_blur, erosion, dilatation, median
};

const RADIUS: u32 = 2;
const IMG: &str = "images/lena.jpg";

#[bench]
fn bench_flou_moyen(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| flou_moyen(&img, RADIUS));
    Ok(())
}

#[bench]
fn bench_flou_moyen_opt(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| optimized_blur(&img, RADIUS));
    Ok(())
}

#[bench]
fn bench_erosion(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| erosion(&img, RADIUS));
    Ok(())
}

#[bench]
fn bench_dilatation(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| dilatation(&img, RADIUS));
    Ok(())
}

#[bench]
fn bench_median(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(IMG);
    let img = image::open(path)?.into_rgba8();

    b.iter(|| median(&img, RADIUS));
    Ok(())
}
