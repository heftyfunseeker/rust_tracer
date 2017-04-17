extern crate rusty_math;
extern crate rand;

use rusty_math::*;
use rand::Rng;
use std::f64::consts;

fn random_in_unit_disk() -> Vec3 {
    let u = &Vec3::new(1f64, 1f64, 0f64);
    let mut p = Vec3::new(2f64, 2f64, 0f64);
    while p.dot(&p) >= 1.0f64 {
        let rand_x = rand::thread_rng().gen_range(0f64, 1f64);
        let rand_y = rand::thread_rng().gen_range(0f64, 1f64);
        p = &(2.0f64 * &Vec3::new(rand_x, rand_y, 0f64)) - u;
    }
    return p;
}

pub struct Camera {
    pub origin:Vec3,
    pub lower_left:Vec3,
    pub horizontal:Vec3,
    pub vertical:Vec3,

    // orthonormal basis for image plane
    pub u:Vec3,
    pub v:Vec3,
    pub w:Vec3,

    pub lens_radius:f64,
}

impl Camera {
    pub fn new(
        pos: &Vec3,
        look_at: &Vec3,
        up:Vec3,
        vertical_fov_degrees:f64,
        aspect_ratio:f64,
        aperture:f64,
        focus_dist:f64,
    ) -> Camera {
        let theta_rads = vertical_fov_degrees * consts::PI / 180f64;
        let half_height = f64::tan(theta_rads / 2f64);
        let half_width = aspect_ratio * half_height;
        let w = (pos - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u).normalize();

        let left = &(half_width * focus_dist * &u);
        let bottom = &(half_height * focus_dist * &v);
        let offset = &(focus_dist * &w);
        return Camera {
            origin: Vec3::new(pos.x, pos.y, pos.z),
            lower_left: &(&(pos - left) - bottom) - offset,
            horizontal: 2f64 * half_width * focus_dist * &u,
            vertical: 2f64 * half_height * focus_dist * &v,
            w: w,
            u: u,
            v: v,
            lens_radius: aperture / 2f64,
        };
    }

    pub fn get_origin(&self) -> Vec3 {
        return Vec3::new(self.origin.x, self.origin.y, self.origin.z);
    }

    pub fn get_ray(&self, s:f64, t:f64) -> Ray {
        let rd = self.lens_radius * &random_in_unit_disk();
        let offset = &(&self.u * rd.x) + &(&self.v * rd.y);
        let a = &(s * &self.horizontal);
        let b = &(t * &self.vertical);
        return Ray::new(
            &self.origin + &offset,
            &(&(&(&self.lower_left + a) + b) - &self.origin) - &offset,
        );
    }
}
