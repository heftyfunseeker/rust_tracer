extern crate rusty_math;
extern crate rand;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rusty_math::*;

use rand::Rng;

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

    // create our material pool
    let mut materials: Vec<Box<renderable::Material>> = Vec::new();
    for material_index in 0..1000 {
        let roll = rand::thread_rng().gen_range(0f64, 1f64);

        if roll < 0.4f64 { // lambertian
            // random color
            let r = rand::thread_rng().gen_range(0f64, 1f64) * rand::thread_rng().gen_range(0f64, 1f64);
            let g = rand::thread_rng().gen_range(0f64, 1f64) * rand::thread_rng().gen_range(0f64, 1f64);
            let b = rand::thread_rng().gen_range(0f64, 1f64) * rand::thread_rng().gen_range(0f64, 1f64);
            materials.push(
                Box::new(
                    materials::Lambertian {
                        albedo: Vec3::new(r, g, b)
                    }
                )
            );
        }
        else if roll < 0.75f64 { // metal
            let r = rand::thread_rng().gen_range(0.5f64, 1f64);
            let g = rand::thread_rng().gen_range(0.5f64, 1f64);
            let b = rand::thread_rng().gen_range(0.5f64, 1f64);
            let fuzz = rand::thread_rng().gen_range(0.01f64, 0.4f64);
            materials.push(
                Box::new(
                    materials::Metal {
                        albedo: Vec3::new(r, g, b),
                        fuzziness: fuzz,
                    }
                )
            );
        }
        else { // dielectric
            materials.push(Box::new(materials::Dielectric { refraction_index: 1.5f64 }));
        }
    }

    let floor_mat = materials::Lambertian { albedo: Vec3::new(0.5f64, 0.5f64, 0.5f64) };
    let center_mat_metal = materials::Metal {
        albedo: Vec3::new(0.70f64, 0.70f64, 0.70f64),
        fuzziness: 0.05f64,
    };
    let center_mat_dielec = materials::Dielectric { refraction_index: 1.5f64 };

    let mut world = RenderList::new();

    for z in 0..25 {
        for x in -10..10 {
            let x_variance = rand::thread_rng().gen_range(1f64, 1.3f64);
            let z_variance = rand::thread_rng().gen_range(1f64, 1.3f64);
            let radius = rand::thread_rng().gen_range(0.1f64, 0.3f64);
            let sphere = shapes::Sphere {
                center: Vec3::new(x as i64 as f64 * x_variance , radius, z as i64 as f64 * -z_variance),
                radius: radius,
            };

            let material_index = rand::thread_rng().gen_range(0, materials.len());
            world.add_sphere(&sphere, &*materials[material_index]);
        }
    }
    // floor
    let sphere = shapes::Sphere {
        center: Vec3::new(0f64,-1000f64, 0f64),
        radius: 1000f64,
    };

    let center_sphere_metal = shapes::Sphere {
        center: Vec3::new(0f64, 1.5f64, -5f64),
        radius: 1.5f64,
    };
    let center_sphere_dielec = shapes::Sphere {
        center: Vec3::new(-2f64, 1.5f64, -7f64),
        radius: 1.5f64,
    };
    world.add_sphere(&sphere, &floor_mat);
    world.add_sphere(&center_sphere_metal, &center_mat_metal);
    world.add_sphere(&center_sphere_dielec, &center_mat_dielec);

    // render setting?
    let image_width_pixels = 400f64;
    let image_height_pixels = 300f64;

    let pos = Vec3::new(0f64, 2f64, 1f64);
    let look_at = Vec3::new(0f64, 1f64, -5f64);
    let camera = Camera::new(
        &pos,
        &look_at,
        Vec3::new(0f64, 1f64, 0f64), // up
        40f64, // fov
        image_width_pixels/image_height_pixels, // aspect ratio
        0.2f64, //aperture
        (&pos - &look_at).length_squared().sqrt(), // focus dist
    );

    let mut output_buffer = RenderBufferI32::new(image_width_pixels as usize, image_height_pixels as usize);

    // render
    {
        let render_settings = renderer::RenderSettings { num_samples_per_pixel: 50 };
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
    ppm_str.push_str(
        &format!(
            "P3\n{} {}\n255\n",
            output_buffer.width,
            output_buffer.height
        )
    );

    let mut pixel_index: usize = 0;
    while pixel_index < output_buffer.buffer.len() {
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
        Err(why) => panic!("{}", why.description()),
        Ok(_) => println!("write to file successful"),
    };
}
