extern crate rusty_math;
extern crate rand;

use rusty_math::*;
use rand::Rng;


pub struct HitRecord {
	pub point: Vec3,
	pub normal: Vec3,
	pub time: f64,
}

impl HitRecord {
	pub fn new() -> HitRecord {
		return HitRecord {
			point: Vec3::new(0f64, 0f64, 0f64),
			normal: Vec3::new(0f64, 0f64, 0f64),
			time:0f64
		};
	}
}

pub struct MaterialInput {
	pub ray:Ray,
	pub point: Vec3,
	pub normal: Vec3,
}

pub struct MaterialOutput {
	pub attenuation:Vec3,
	pub scattered:Ray,
}

pub trait Material {
	fn apply(&self, input:MaterialInput) -> MaterialOutput;
}

pub trait Renderable {
	fn hit(&self, ray: &Ray, time_min: f64, time_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct RenderList {
	pub renderables: Vec<Box<Renderable>>,
}

impl Renderable for RenderList {
	fn hit(&self, ray: &Ray, time_min: f64, time_max: f64, hit_record: &mut HitRecord) -> bool {
		let mut ray_did_hit = false;
		let mut closest_time = time_max;
		for renderable in &self.renderables {
			if renderable.hit(ray, time_min, closest_time, hit_record) {
				closest_time = hit_record.time;
				ray_did_hit = true;
			}
		}
		return ray_did_hit;
	}
}

/*pub mod materials {
	use rusty_math::*;
	use super::Material;
	use super::MaterialInput;
	use super::MaterialOutput;


	pub struct Lambertian {
		pub albedo:Vec3,
	}

	impl Material for Lambertian {
		fn apply(&self, input:MaterialInput) -> MaterialOutput {
			let target = &(&input.point + &input.normal) + &super::random_in_unit_sphere();
			let dir = &target - &input.point;
			return MaterialOutput {
				scattered: Ray::new(input.point, dir),
				attenuation: Vec3::new(self.albedo.x, self.albedo.y, self.albedo.z),
			};
		}
	}
}*/

pub mod shapes {
	use rusty_math::*;
	use super::Renderable;
	use super::HitRecord;

	//
	// Sphere
	//
	pub struct Sphere {
		pub center: Vec3,
		pub radius: f64,
	}

	impl Renderable for Sphere {
		fn hit(&self, ray: &Ray, time_min: f64, time_max: f64, hit_record: &mut HitRecord) -> bool {
		    let offset_origin = &ray.origin - &self.center;
		    let a = ray.dir.dot(&ray.dir);
		    let b = 2f64 * offset_origin.dot(&ray.dir);
		    let c = offset_origin.dot(&offset_origin) - self.radius * self.radius;
		    let discriminant: f64 = &b * &b - 4f64 * &a * &c;

			if discriminant <= 0f64 {
				return false;
			}

			let discriminant_root = discriminant.sqrt();
			let mut time: f64 = (-b - discriminant_root) / (2.0f64 * &a);

			// check if in range
			if time >= time_max || time <= time_min {
				time = (-b + discriminant_root) / (2.0f64 * &a);
				if time >= time_max || time <= time_min {
					return false; // not in range
				}
			}

			hit_record.time = time;
			hit_record.point = ray.point_at(time);
			hit_record.normal = &(&hit_record.point - &self.center) / self.radius;
			return true;
		}
	}
}
