use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut ppm_str = String::new();

    let nx = 200;
    let ny = 100;

    // write header
    ppm_str.push_str(&format!("P3\n{} {}\n255\n", nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r: f32 = (i as f32) / nx as f32;
            let g: f32 = (j as f32) / ny as f32;
            let b: f32 = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            ppm_str.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }

    // write to file
    let path = Path::new("image.ppm");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open file: {}", why.description()),
        Ok(file) => file,
    };
    match file.write_all(ppm_str.as_bytes()) {
        Err(why) => {
            panic!("{}", why.description())
        },
        Ok(_) => println!("write to file successful")
    }
}
