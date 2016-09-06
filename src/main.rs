extern crate rusty_math;

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
            let v = rusty_math::Vec3 {
                x: (i as f64) / (nx as f64),
                y: (j as f64) / (ny as f64),
                z: 0.2f64
            };

            let ir = (255.99 * v.x) as i32;
            let ig = (255.99 * v.y) as i32;
            let ib = (255.99 * v.z) as i32;


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
