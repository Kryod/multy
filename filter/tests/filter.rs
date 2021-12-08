use filter::Buffer;

fn open_files(source: &str, expected: &str) -> (Buffer, Buffer) {
    let expected = image::open(expected).unwrap().into_rgba8();
    let source = image::open(source).unwrap().into_rgba8();
    (source, expected)
}

fn compare_buffer(computed: Buffer, expected: Buffer, err_output: &str) {
    let mismatch = computed.pixels()
        .zip(expected.pixels())
        .filter(|(lhs, rhs)| lhs != rhs)
        .count();

    if mismatch > 0 {
        let dest = "tests/error/";
        std::fs::create_dir_all(dest).unwrap();
        let output = format!("{}{}", dest, err_output);

        filter::compare(&computed, &expected).unwrap()
            .save(&output).unwrap();

        panic!("test fail! found {} pixels differents. see {} for more details", mismatch, output);
    }
}

#[test]
fn blur() {
    let (source, expected) = open_files("tests/images/grid.png", "tests/expected/blur.png");
    let computed = filter::blur(&source, 1);
    compare_buffer(computed, expected, "blur.png");
}

#[test]
fn dilate() {
    let (source, expected) = open_files("tests/images/grid.png", "tests/expected/dilate.png");
    let computed = filter::dilate(&source, 1);
    compare_buffer(computed, expected, "dilate.png");
}

#[test]
fn erode() {
    let (source, expected) = open_files("tests/images/grid.png", "tests/expected/erode.png");
    let computed = filter::erode(&source, 1);
    compare_buffer(computed, expected, "erode.png");
}

#[test]
fn local_contrast() {
    let (source, expected) = open_files("tests/images/noise.png", "tests/expected/local_contrast.png");
    let computed = filter::local_contrast(&source, 32, 120);
    compare_buffer(computed, expected, "local_contrast.png");
}

#[test]
fn median_blur() {
    let (source, expected) = open_files("tests/images/noise.png", "tests/expected/median_blur.png");
    let computed = filter::median_blur(&source, 1);
    compare_buffer(computed, expected, "median_blur.png");
}

#[test]
fn min_max() {
    let (source, expected) = open_files("tests/images/noise.png", "tests/expected/min_max.png");
    let computed = filter::min_max(&source, 1);
    compare_buffer(computed, expected, "min_max.png");
}
