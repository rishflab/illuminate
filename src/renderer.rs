pub mod core;
pub mod pathtracer;
pub mod shaders;
pub mod cpu;

use self::core::backend::BackendState;
use crate::window::WindowState;
use crate::scene::Scene;

use gfx_hal::{Backend, format, image};

const ENTRY_NAME: &str = "main";

const WORK_GROUP_SIZE: u32 = 8;

const RAY_SAMPLES: u32 = 4;

const COLOR_RANGE: image::SubresourceRange = image::SubresourceRange {
    aspects: format::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

pub trait Renderer<B: Backend>{
    unsafe fn new(backend: BackendState<B>, window: WindowState, scene: &Scene) -> Self;
    fn render(&mut self, scene: &Scene);
}
