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

use renderable::materials;

mod camera;
use camera::Camera;

mod render_buffer;
use render_buffer::RenderBufferI32;

mod renderer;

fn main() {
	// setup the world
	// all materials must be declared before the renderlist references them
	let lambertian_red = materials::Lambertian {albedo: Vec3::new(0.8f64, 0.3f64, 0.3f64)};
	let lambertian_blue = materials::Lambertian {albedo: Vec3::new(0.8f64, 0.8f64, 0f64)};
	let metal_1 = materials::Metal {albedo: Vec3::new(0.8f64, 0.6f64, 0.2f64), fuzziness: 0.8f64};
	let metal_2 = materials::Metal {albedo: Vec3::new(0.8f64, 0.8f64, 0.8f64), fuzziness: 0.3f64};
	let dielectric = materials::Dielectric {refractionIndex: 1.5f64};

	let mut world = RenderList::new();
	let small_sphere = shapes::Sphere {center: Vec3::new(0f64, 0f64, -1f64), radius: 0.5f64};
	let right_sphere  = shapes::Sphere {center: Vec3::new(-1f64, 0f64, -1f64), radius: 0.5f64};
	let left_sphere  = shapes::Sphere {center: Vec3::new(1f64, 0f64, -1f64), radius: 0.5f64};
	let big_sphere = shapes::Sphere {center: Vec3::new(0f64, -100.5f64, -1f64), radius: 100f64};
	world.add_sphere(&small_sphere, &lambertian_red);
	world.add_sphere(&big_sphere, &lambertian_blue);
	world.add_sphere(&right_sphere, &dielectric);
	world.add_sphere(&left_sphere, &metal_1);
	let camera = Camera::new();
	let mut output_buffer = RenderBufferI32::new(200, 100);

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
