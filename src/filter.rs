use std::path::PathBuf;

use image::{ Pixel, GenericImageView };


pub fn flou_moyen(path: PathBuf) -> PathBuf {

    let mut to_save = PathBuf::from("images");
    let new_path = path.file_stem().unwrap().to_str().unwrap().to_owned() 
    + &String::from("_flou_moyen.") 
    + path.extension().unwrap().to_str().unwrap();
    //println!("{}", new_path);
    to_save.push(new_path);
    let img = image::open(path).unwrap();
    let mut buffer = image::ImageBuffer::new(img.width(), img.height());

    let radius = 2;

    let width = img.width();
    let height = img.height();

    for x in 0..width {
        //let x = x as i64;
        for y in 0..height {
            //let y = y as i64;

            let mut sum = (0u32, 0u32, 0u32, 0u32);
            let mut neighbours = 0u32;
            for neighbour_x in (x.saturating_sub(radius))..(x.saturating_add(radius + 1)) {
                for neighbour_y in (y.saturating_sub(radius))..(y.saturating_add(radius + 1)) {
                    if !(neighbour_x >= width || neighbour_y >= height) {
                        let p = img.get_pixel(neighbour_x, neighbour_y).channels4();
                        sum.0 += p.0 as u32;
                        sum.1 += p.1 as u32;
                        sum.2 += p.2 as u32;
                        sum.3 += p.3 as u32;
                        neighbours += 1;
                    }
                }
            }

            let avg = [(sum.0 / neighbours) as u8, (sum.1 / neighbours) as u8, (sum.2 / neighbours) as u8, (sum.3 / neighbours) as u8];
            buffer.put_pixel(x, y, image::Rgba(avg));
        }
    }
    println!("{:?}", &to_save);
    //let _ = std::fs::File::create(&to_save).unwrap();
    buffer.save(&to_save).unwrap();

    to_save
}

#[test]
fn test_flou_moyen() {
    flou_moyen(PathBuf::from("images/lena2.jpg"));
}