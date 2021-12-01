use filter::Buffer;
const RADIUS: u32 = 1;

fn global_test(source: &str, expected: &str, algo: fn(&Buffer, u32) -> Buffer) {
    let img = image::open(source).unwrap().into_rgba8();
    let computed = algo(&img, RADIUS);

    let expected_img = image::open(expected).unwrap().into_rgba8();
    // assert!(computed.pixels().zip(expected.pixels()).all(|(lhs, rhs)| lhs == rhs))
    let mismatch = computed.pixels()
        .zip(expected_img.pixels())
        .filter(|(lhs, rhs)| lhs != rhs)
        .count();

    if mismatch > 0 {
        let dest = "tests/error/";
        let fname = std::path::Path::new(expected)
        .file_name().unwrap()
        .to_str().unwrap();

        std::fs::create_dir_all(dest).unwrap();
        let output = format!("{}{}", dest, fname);

        filter::compare(&computed, &expected_img).unwrap()
            .save(&output).unwrap();

        panic!("test fail! found {} pixels differents. see {} for more details", mismatch, output);
    }
}

#[test]
fn blur() {
    global_test("tests/images/grid.png", "tests/expected/blur.png", filter::blur)
}

#[test]
fn dilate() {
    global_test("tests/images/grid.png", "tests/expected/dilate.png", filter::dilate)
}

#[test]
fn erode() {
    global_test("tests/images/grid.png", "tests/expected/erode.png", filter::erode)
}

#[test]
fn median_blur() {
    global_test("tests/images/noise.png", "tests/expected/median_blur.png", filter::median_blur)
}

#[test]
fn min_max() {
    global_test("tests/images/noise.png", "tests/expected/min_max.png", filter::min_max)
}
