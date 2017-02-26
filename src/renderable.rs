extern crate rusty_math;
extern crate rand;

use rusty_math::*;
use rand::Rng;
use std::usize;

pub struct HitRecord {
	pub ray: Ray,
	pub index: usize,
	pub time: f64
}

impl HitRecord {
	pub fn new() -> HitRecord {
		return HitRecord {
			ray: Ray::new(Vec3::new(0f64, 0f64, 0f64), Vec3::new(0f64, 0f64, 0f64)),
			index: 0,
			time: 0f64
		};
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

pub trait Renderable {
	// the time along the ray where the intersection occurs
	fn get_hit_time(&self, ray: &Ray, time_min: f64, time_max: f64) -> f64;
}

// only supports spheres right now (avoid dynamic dispatch with explicit vector template)
pub struct RenderList<'a> {
	// list of spheres to render with parallel array
	m_spheres: Vec<shapes::Sphere>,
	m_sphere_materials: Vec<&'a Material>, // dynamic dispatch on materials
}

impl<'a> RenderList<'a> {
		pub fn new() -> RenderList<'a> {
			return RenderList {
				m_spheres: Vec::new(),
				m_sphere_materials: Vec::new()
			};
		}

		//@nicco: maybe return handle/index if we need it later?
		pub fn add_sphere(& mut self, sphere: &shapes::Sphere, material: &'a Material) {
			//@nicco: derive copy for vec3 and sphere
			let sphere_copy = shapes::Sphere {
				center: Vec3::new(sphere.center.x, sphere.center.y, sphere.center.z),
				radius: sphere.radius
			};
			self.m_spheres.push(sphere_copy);
			self.m_sphere_materials.push(material);
		}

	    pub fn try_get_hit_record(&self, ray: &Ray, time_min: f64, time_max: f64, hit_record: &mut HitRecord) -> bool {
			let mut closest_hit_index:usize = usize::MAX;
			let mut closest_time = time_max;
			for index in 0..self.m_spheres.len() {
				let sphere = &self.m_spheres[index];
				let hit_time = sphere.get_hit_time(ray, time_min, closest_time);
				if hit_time > time_min {
					closest_time = hit_time;
					closest_hit_index = index;
				}
			}
			if closest_hit_index == usize::MAX {
				return false;
			}
			hit_record.time = closest_time;
			hit_record.index = closest_hit_index;
			hit_record.ray = Ray::new(Vec3::new(ray.origin.x, ray.origin.y, ray.origin.z), Vec3::new(ray.dir.x, ray.dir.y, ray.dir.z));
			return true;
		}

		pub fn get_material_package(&self, ray: &Ray, hit_time: f64, hit_index:usize) -> MaterialPackage<'a> {
				let sphere = &self.m_spheres[hit_index];
				let material_input = MaterialInput {
					point: ray.point_at(hit_time),
					normal: &(&ray.point_at(hit_time) - &sphere.center) / sphere.radius,
				};

				let material_package = MaterialPackage {
					material: self.m_sphere_materials[hit_index],
					material_input: material_input,
				};
				return material_package;
		}
}

pub mod shapes {
	use rusty_math::*;
	use super::Renderable;
	//
	// Sphere
	//
	pub struct Sphere {
		pub center: Vec3,
		pub radius: f64,
	}

	impl Renderable for Sphere {
		fn get_hit_time(&self, ray: &Ray, time_min: f64, time_max: f64) -> f64 {
		    let offset_origin = &ray.origin - &self.center;
		    let a = ray.dir.dot(&ray.dir);
		    let b = 2f64 * offset_origin.dot(&ray.dir);
		    let c = offset_origin.dot(&offset_origin) - self.radius * self.radius;
		    let discriminant: f64 = &b * &b - 4f64 * &a * &c;

			if discriminant <= 0f64 {
				return time_min;
			}

			let discriminant_root = discriminant.sqrt();
			let mut time: f64 = (-b - discriminant_root) / (2.0f64 * &a);

			// check if in range
			if time >= time_max || time <= time_min {
				time = (-b + discriminant_root) / (2.0f64 * &a);
				if time >= time_max || time <= time_min {
					return time_min; // not in range
				}
			}
			return time;
		}
	}
}

pub struct MaterialInput {
	pub point: Vec3,
	pub normal: Vec3,
}

pub struct MaterialOutput {
	pub attenuation:Vec3,
	pub scattered:Ray,
}

impl MaterialOutput {
	pub fn new() -> MaterialOutput {
		return MaterialOutput {
			attenuation: Vec3::new(0f64, 0f64, 0f64),
			scattered: Ray::new(Vec3::new(0f64, 0f64, 0f64), Vec3::new(0f64, 0f64, 0f64))
		};
	}
}

pub struct MaterialPackage<'a> {
	pub material: &'a Material,
	pub material_input: MaterialInput,
}

pub trait Material {
	fn apply(&self, input: &MaterialInput, output: &mut MaterialOutput) -> bool;
}

pub mod materials {
	use rusty_math::*;
	use super::Material;
	use super::MaterialInput;
	use super::MaterialOutput;

	pub struct Lambertian {
		pub albedo:Vec3,
	}

	impl Material for Lambertian {
		fn apply(&self, input: &MaterialInput, output: &mut MaterialOutput) -> bool {
			let target = &(&input.point + &input.normal) + &super::random_in_unit_sphere();
			let dir = &target - &input.point;
			output.scattered = Ray::new(Vec3::new(input.point.x, input.point.y, input.point.z), dir);
			output.attenuation = Vec3::new(self.albedo.x, self.albedo.y, self.albedo.z);
			return true;
		}
	}
}
