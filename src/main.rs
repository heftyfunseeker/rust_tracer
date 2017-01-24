extern crate rusty_math;
extern crate rand;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rusty_math::*;

mod renderable;
use renderable::RenderList;
use renderable::shapes;

mod camera;
use camera::Camera;

mod render_buffer;
use render_buffer::RenderBufferI32;

mod renderer;


fn main() {
	// setup the world
	let camera = Camera::new();
	let mut output_buffer = RenderBufferI32::new(200, 100);

	let mut world = RenderList {renderables: Vec::new()};
	let small_sphere = shapes::Sphere {center: Vec3::new(0f64, 0f64, -1f64), radius: 0.5f64};
	let big_sphere = shapes::Sphere {center: Vec3::new(0f64, -100.5f64, -1f64), radius: 100f64};
	world.renderables.push(Box::new(small_sphere));
	world.renderables.push(Box::new(big_sphere));


	// render
	{
		let render_settings = renderer::RenderSettings {
			num_samples_per_pixel: 100,
		};
		// create the package to render
		let mut render_package = renderer::RenderPackage {
			render_list: &world,
			camera: &camera,
			output_buffer: &mut output_buffer,
		};


		renderer::render(&mut render_package, &render_settings);
	}

    // write header
    let mut ppm_str = String::new();
    ppm_str.push_str(&format!("P3\n{} {}\n255\n", output_buffer.width, output_buffer.height));

	let mut pixel_index:usize = 0;
	while pixel_index < output_buffer.buffer.len()
	{
		let ir = output_buffer.buffer[pixel_index];
		let ig = output_buffer.buffer[pixel_index + 1];
		let ib = output_buffer.buffer[pixel_index + 2];

	    ppm_str.push_str(&format!("{} {} {}\n", ir, ig, ib));

		pixel_index += 3;
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
    };
}
