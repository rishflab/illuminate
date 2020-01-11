use gfx_hal::{pso};

fn shader_to_spirv(glsl: &str) -> Vec<u32> {
    let file = glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Compute).unwrap();
    pso::read_spirv(file).unwrap()
}

pub fn camera_ray_generator_spirv() -> Vec<u32> {
    let glsl = include_str!("shaders/camera_rays.comp");
    shader_to_spirv(glsl)
}

pub fn aabb_calculator_spirv() -> Vec<u32> {
    let glsl = include_str!("shaders/calculate_aabbs.comp");
    shader_to_spirv(glsl)
}

pub fn ray_triangle_intersector_spirv() -> Vec<u32> {
    let glsl = include_str!("shaders/bounces.comp");
    shader_to_spirv(glsl)
}

pub fn vertex_skinner_spirv() -> Vec<u32> {
    let glsl = include_str!("shaders/vertex_skinning.comp");
    shader_to_spirv(glsl)
}

pub fn accumulator_spirv() -> Vec<u32> {
    let glsl = include_str!("shaders/accumulate_intersections.comp");
    shader_to_spirv(glsl)
}

