use image::{ Pixel, GenericImageView };

pub fn flou_moyen() {
    let img = image::open("lena2.jpg").unwrap();
    let mut buffer = image::ImageBuffer::new(img.width(), img.height());

    let radius = 2;

    let width = img.width();
    let height = img.height();

    for x in 0..width {
        let x = x as i64;
        for y in 0..height {
            let y = y as i64;

            let mut sum = 0_u64;
            let mut neighbours = 0;
            for neighbour_x in (x - radius)..(x + radius + 1) {
                for neighbour_y in (y - radius)..(y + radius + 1) {
                    if !(neighbour_x < 0 || neighbour_x >= width as i64 || neighbour_y < 0 || neighbour_y >= height as i64) {
                        let p = img.get_pixel(neighbour_x as u32, neighbour_y as u32);
                        sum += p.to_luma()[0] as u64;
                        neighbours += 1;
                    }
                }
            }

            let avg = (sum as f32 / neighbours as f32) as u8;
            buffer.put_pixel(x as u32, y as u32, image::Rgb([avg, avg, avg]));
        }
    }

    buffer.save("result.jpg").unwrap();
}