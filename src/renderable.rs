extern crate rusty_math;
extern crate rand;

use rusty_math::*;
use rand::Rng;
use std::usize;

pub struct HitRecord {
    pub ray: Ray,
    pub index: usize,
    pub time: f64,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        return HitRecord {
                   ray: Ray::new(Vec3::new(0f64, 0f64, 0f64), Vec3::new(0f64, 0f64, 0f64)),
                   index: 0,
                   time: 0f64,
               };
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let u = &Vec3::new(1f64, 1f64, 1f64);
    let mut p = Vec3::new(2f64, 2f64, 2f64);
    while {
              p.length_squared() >= 1.0f64
          } {
        let rand_x = rand::thread_rng().gen_range(0f64, 1f64);
        let rand_y = rand::thread_rng().gen_range(0f64, 1f64);
        let rand_z = rand::thread_rng().gen_range(0f64, 1f64);
        p = &(2.0f64 * &Vec3::new(rand_x, rand_y, rand_z)) - u;
    }
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
                   m_sphere_materials: Vec::new(),
               };
    }

    //@nicco: maybe return handle/index if we need it later?
    pub fn add_sphere(&mut self, sphere: &shapes::Sphere, material: &'a Material) {
        //@nicco: derive copy for vec3 and sphere
        let sphere_copy = shapes::Sphere {
            center: Vec3::new(sphere.center.x, sphere.center.y, sphere.center.z),
            radius: sphere.radius,
        };
        self.m_spheres.push(sphere_copy);
        self.m_sphere_materials.push(material);
    }

    pub fn try_get_hit_record(&self,
                              ray: &Ray,
                              time_min: f64,
                              time_max: f64,
                              hit_record: &mut HitRecord)
                              -> bool {
        let mut closest_hit_index: usize = usize::MAX;
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
        hit_record.ray = Ray::new(Vec3::new(ray.origin.x, ray.origin.y, ray.origin.z),
                                  Vec3::new(ray.dir.x, ray.dir.y, ray.dir.z));
        return true;
    }

    pub fn get_material_package(&self,
                                ray: &Ray,
                                hit_time: f64,
                                hit_index: usize)
                                -> MaterialPackage<'a> {
        let sphere = &self.m_spheres[hit_index];
        //@nicco: make the vec3 and ray classes implement the copy trait
        let origin = Vec3::new(ray.origin.x, ray.origin.y, ray.origin.z);
        let dir = Vec3::new(ray.dir.x, ray.dir.y, ray.dir.z);
        let material_input = MaterialInput {
            point: ray.point_at(hit_time),
            normal: &(&ray.point_at(hit_time) - &sphere.center) / sphere.radius,
            incoming_ray: Ray::new(origin, dir),
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
    pub incoming_ray: Ray,
    pub point: Vec3,
    pub normal: Vec3,
}

pub struct MaterialOutput {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

impl MaterialOutput {
    pub fn new() -> MaterialOutput {
        return MaterialOutput {
                   attenuation: Vec3::new(0f64, 0f64, 0f64),
                   scattered: Ray::new(Vec3::new(0f64, 0f64, 0f64), Vec3::new(0f64, 0f64, 0f64)),
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
    extern crate rand;
    use rusty_math::*;
    use super::Material;
    use super::MaterialInput;
    use super::MaterialOutput;
    use rand::Rng;

    pub struct Lambertian {
        pub albedo: Vec3,
    }

    impl Material for Lambertian {
        fn apply(&self, input: &MaterialInput, output: &mut MaterialOutput) -> bool {
            let target = &(&input.point + &input.normal) + &super::random_in_unit_sphere();
            let dir = &target - &input.point;

            output.scattered = Ray::new(Vec3::new(input.point.x, input.point.y, input.point.z),
                                        dir);
            output.attenuation = Vec3::new(self.albedo.x, self.albedo.y, self.albedo.z);

            return true;
        }
    }

    pub struct Metal {
        pub albedo: Vec3,
        pub fuzziness: f64,
    }

    impl Material for Metal {
        fn apply(&self, input: &MaterialInput, output: &mut MaterialOutput) -> bool {
            // get reflected
            let unit_dir = &(input.incoming_ray.dir.normalize());
            let surface_normal = &input.normal;
            let mut reflected = unit_dir - &(2f64 * unit_dir.dot(&surface_normal) * surface_normal);
            reflected = &reflected + &(self.fuzziness * &super::random_in_unit_sphere());

            if reflected.dot(surface_normal) > 0f64 {
                let hit_point = Vec3::new(input.point.x, input.point.y, input.point.z);
                output.scattered = Ray::new(hit_point, reflected);
                output.attenuation = Vec3::new(self.albedo.x, self.albedo.y, self.albedo.z);
                return true;
            }
            return false;
        }
    }

    pub struct Dielectric {
        pub refraction_index: f64,
    }

    fn schlick(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1f64 - refraction_index) / (1f64 + refraction_index);
        r0 *= r0;
        return r0 + (1f64 - r0) * (1f64 - cosine).powf(5f64);
    }

    impl Material for Dielectric {
        fn apply(&self, input: &MaterialInput, output: &mut MaterialOutput) -> bool {
            let outward_normal: Vec3;

            // get reflected
            let dir = &(input.incoming_ray.dir);
            let dir_normalize = dir.normalize();
            let surface_normal = &input.normal;

            let reflected = &dir_normalize -
                            &(2f64 * dir_normalize.dot(&surface_normal) * surface_normal);
            let ni_over_nt: f64;
            let cosine: f64;
            let reflect_chance: f64;

            output.attenuation = Vec3::new(1f64, 1f64, 1f64);

            let dir_dot_normal = dir.dot(&input.normal);
            let dir_length = dir.length_squared().sqrt();

            if dir_dot_normal > 0f64 {
                outward_normal = -1f64 * surface_normal;
                ni_over_nt = self.refraction_index;
                cosine = self.refraction_index * dir_dot_normal / dir_length;
            } else {
                outward_normal = Vec3::new(surface_normal.x, surface_normal.y, surface_normal.z);
                ni_over_nt = 1.0f64 / self.refraction_index;
                cosine = -dir_dot_normal / dir_length;
            }

            let hit_point = Vec3::new(input.point.x, input.point.y, input.point.z);

            // calculate refraction
            // ray dir, outward_normal, ni_over_nt, refracted
            let dt = dir_normalize.dot(&outward_normal);
            let discriminant = 1f64 - ni_over_nt * ni_over_nt * (1f64 - dt * dt);
            let mut refracted: Vec3 = Vec3::new(0f64, 0f64, 0f64);
            if discriminant > 0f64 {
                // fuck this syntax
                refracted = &(ni_over_nt * &(&dir_normalize - &(dt * &outward_normal))) -
                            &(&outward_normal * discriminant.sqrt());
                reflect_chance = schlick(cosine, self.refraction_index);
            } else {
                // we reflect
                reflect_chance = 1f64;
            }

            // roll to see if we reflect
            let roll = rand::thread_rng().gen_range(0f64, 1f64);
            if roll < reflect_chance {
                output.scattered = Ray::new(hit_point, reflected);
            } else {
                output.scattered = Ray::new(hit_point, refracted);
            }

            return true;
        }
    }

}
