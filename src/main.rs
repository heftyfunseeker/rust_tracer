extern crate rusty_math;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rusty_math::*;

fn color(ray: &Ray) -> Vec3 {
    let t = 0.5f64 * (ray.dir.normalize().y + 1.0f64);
    let white = Vec3::new(1f64, 1f64, 1f64);
    let blue = Vec3::new(0.5f64, 0.7f64, 1.0f64);
    return &((1.0f64 - t) * &white) + &(t * &blue);
}

fn main() {
    let nx = 200;
    let ny = 100;
    let lower_left = Vec3::new(-2f64,-1f64,-1f64);
    let horizontal = Vec3::new(4f64,0f64,0f64);
    let vertical = Vec3::new(0f64,2f64,0f64);
    let origin = Vec3::new(0f64,0f64,0f64);

    let mut r = Ray::new(origin, Vec3::new(0f64,0f64,0f64));
    
    // write header
    let mut ppm_str = String::new();
    ppm_str.push_str(&format!("P3\n{} {}\n255\n", nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let u_offset = u * &horizontal;
            let v_offset = v * &vertical;

            r.dir = &lower_left + &(&u_offset + &v_offset);
            let c = color(&r);

            let ir = (255.99 * c.x) as i32;
            let ig = (255.99 * c.y) as i32;
            let ib = (255.99 * c.z) as i32;

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
