extern crate rusty_math;
extern crate rand;

use rusty_math::*;

use renderable::RenderList;
use renderable::HitRecord;
use renderable::Renderable;
use camera::Camera;
use render_buffer::RenderBufferI32;
use rand::Rng;
use std::f64;

pub struct RenderPackage<'a> {
	pub render_list: &'a RenderList,
	pub camera: &'a Camera,
	pub output_buffer: &'a mut RenderBufferI32,
}

pub struct RenderSettings {
	pub num_samples_per_pixel:i32,
}

pub fn render(render_package: &mut RenderPackage, render_settings: &RenderSettings) {
	let num_pixels_y = render_package.output_buffer.height;
	let num_pixels_x = render_package.output_buffer.width;

	let camera = render_package.camera;

    for y in (0..num_pixels_y).rev() {
        for x in 0..num_pixels_x {
			let mut c = Vec3::new(0f64, 0f64, 0f64);
			for s in 0..render_settings.num_samples_per_pixel {
				let rand_offset_x = rand::thread_rng().gen_range(0f64,1f64);
				let rand_offset_y = rand::thread_rng().gen_range(0f64,1f64);

	            let u = (x as f64 + rand_offset_x) / (num_pixels_x as f64);
	            let v = (y as f64 + rand_offset_y) / (num_pixels_y as f64);

				let r = camera.get_ray(u, v);
	             c += &color(&r, &render_package.render_list);
			}
			c /= render_settings.num_samples_per_pixel as f64;
			c.x = c.x.sqrt();
			c.y = c.y.sqrt();
			c.z = c.z.sqrt();

            let ir = (255.99 * c.x) as i32;
            let ig = (255.99 * c.y) as i32;
            let ib = (255.99 * c.z) as i32;

			render_package.output_buffer.push_pixel(ir, ig, ib);
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
	let mut p;

	while {
		let rand_x = rand::thread_rng().gen_range(0f64,1f64);
		let rand_y = rand::thread_rng().gen_range(0f64,1f64);
		let rand_z = rand::thread_rng().gen_range(0f64,1f64);
		p = &(2.0f64 * &Vec3::new(rand_x, rand_y, rand_z)) - &Vec3::new(1f64, 1f64, 1f64);
		p.length_squared() >= 1.0f64
	} {}

	return p;
}

fn color(ray: &Ray, render_list: &RenderList) -> Vec3 {
	// render the list
	let mut hit_record = HitRecord::new();
	if render_list.hit(ray, 0.001f64, f64::MAX, &mut hit_record) {
		let target = &(&hit_record.point + &hit_record.normal) + &random_in_unit_sphere();
		let target_dir = &target - &hit_record.point;

		return 0.5f64 * &color(&Ray::new(hit_record.point, target_dir), &render_list);
	}

	// else color the background with a gradient
    let white = Vec3::new(1f64, 1f64, 1f64);
    let blue = Vec3::new(0.5f64, 0.7f64, 1.0f64);
	let t = 0.5f64 * ray.dir.normalize().y + 1.0f64;
    return &((1.0f64 - t) * &white) + &(t * &blue);
}
