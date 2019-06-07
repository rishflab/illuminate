pub mod window;
pub mod device;
pub mod backend;
pub mod swapchain;
pub mod pipeline;
pub mod buffer;
pub mod descriptor;
pub mod framebuffer;
pub mod scene;
pub mod camera_ray_generator;
pub mod basic;
pub mod staged;
pub mod ray_triangle_intersector;
pub mod types;
pub mod accumulator;

use crate::renderer::scene::Scene;
use crate::renderer::backend::BackendState;
use crate::renderer::window::WindowState;

use gfx_hal::{Backend, format, image};
use gfx_hal::window::Extent2D;

const DIMS: Extent2D = Extent2D {
    width: 800,
    height: 800,
};

const ENTRY_NAME: &str = "main";

const COLOR_RANGE: image::SubresourceRange = image::SubresourceRange {
    aspects: format::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

pub trait Renderer<B: Backend>{
    unsafe fn new(backend: BackendState<B>, window: WindowState, scene: &Scene) -> Self;
    fn render(&mut self, scene: &Scene);
}