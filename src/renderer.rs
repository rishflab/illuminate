pub mod scene;
pub mod core;
pub mod basic_pathtracer;
pub mod staged_pathtracer;

use self::scene::Scene;
use self::core::backend::BackendState;
use crate::window::WindowState;

use gfx_hal::{Backend, format, image};
use gfx_hal::window::Extent2D;

const ENTRY_NAME: &str = "main";

const WORK_GROUP_SIZE: u32 = 8;

const COLOR_RANGE: image::SubresourceRange = image::SubresourceRange {
    aspects: format::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

pub trait Renderer<B: Backend>{
    unsafe fn new(backend: BackendState<B>, window: WindowState, scene: &Scene) -> Self;
    fn render(&mut self, scene: &Scene);
}