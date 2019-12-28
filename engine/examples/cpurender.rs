extern crate engine;
extern crate image;

use engine::renderer::cpu::*;
use engine::scene::camera::Camera;
use nalgebra_glm as glm;
use glm::{vec3, Vec3, quat_look_at};
use nalgebra_glm::mat4_to_mat3;
use image::{RgbImage, ImageBuffer};
use image::hdr::RGBE8Pixel;
use image::ColorType::RGB;


fn main() {

    let width = 800;
    let height = 800;

    let rays = generate_rays((width, height));

    let triangle = Triangle (
        vec3(-0.5, 0.0, -5.0),
        vec3(0.5, 0.0, -5.0),
        vec3(0.0, 1.0, -5.0)
    );

    let mut img: RgbImage = ImageBuffer::new(width, height);

    for ray in rays {
        let intersection = intersect_triangle(&ray, &triangle);
        if intersection.a != -1.0 {
            img.put_pixel(ray.index.0, ray.index.1, image::Rgb([255, 0, 0]));
        } else {
            img.put_pixel(ray.index.0, ray.index.1, image::Rgb([0, 0, 0]));
        }
    }

    img.save("logs/render.png").unwrap();

}