extern crate engine;
extern crate image;

use engine::renderer::cpu::*;
use engine::scene::camera::Camera;
use nalgebra_glm as glm;
use glm::{vec3, Vec3, quat_look_at};
use nalgebra_glm::{mat4_to_mat3, vec4};
use image::{RgbImage, ImageBuffer};
use image::hdr::RGBE8Pixel;
use image::ColorType::RGB;
use engine::scene::light::PointLight;
use rayon::prelude::*;


fn main() {

    let width = 1280;
    let height = 720;

    let up_vec = vec3(0.0, 1.0, 0.0);

    let cam_pos = vec3(0.0, 2.0, 0.0);
    let cam_rot = quat_look_at(&(vec3(0.0, 0.0, -3.0) + cam_pos),&up_vec);

    let rays = generate_camera_rays((width, height));

    let rays = transform_camera_rays(rays, &cam_pos, &cam_rot);

    let triangle = Triangle (
        vec3(0.0, 0.0, -3.0),
        vec3(1.0, 0.0, -3.0),
        vec3(0.0, 1.0, -3.0)
    );

    let tris = vec![triangle];

    let light = PointLight {
        position: vec4(0.0, 2.0, 0.0, 0.0),
        intensity: 20.0,
    };

    let lights = vec![light];

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let pixels: Vec<(u32, u32, u8)> = rays.par_iter().map(|ray|{
        let shade = (255.0 * trace_ray(ray, &tris, &lights)) as u8;
        (ray.index.0, ray.index.1, shade)
    }).collect();

    for (x, y, shade) in pixels {
        img.put_pixel(x, y, image::Rgb([shade, shade, shade]));
    }

    img.save("logs/render.png").unwrap();
}