extern crate rusty_math;

use rusty_math::*;

pub struct Camera {
	pub origin:Vec3,
	pub lower_left:Vec3,
	pub horizontal:Vec3,
	pub vertical:Vec3,
}

impl Camera {
	pub fn new() -> Camera {
		return Camera {
				origin: Vec3::new(0f64, 0f64, 0f64),
				lower_left: Vec3::new(-2f64, -1f64, -1f64),
				horizontal: Vec3::new(4f64, 0f64, 0f64),
				vertical: Vec3::new(0f64, 2f64, 0f64)
		};
	}

	pub fn get_origin(&self) -> Vec3 {
		return Vec3::new(self.origin.x, self.origin.y, self.origin.z);
	}

	pub fn get_ray(&self, u:f64, v:f64) -> Ray {
		return Ray::new(
			self.get_origin(),
			&(&(&self.lower_left + &(u * &self.horizontal)) + &(v * &self.vertical)) - &self.origin
		);
	}
}
