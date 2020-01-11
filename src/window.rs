use std::string::ToString;
use winit::{
    window::WindowBuilder,
};

pub struct Dims {
    pub width: u32,
    pub height: u32,
}

pub const DIMS: Dims = Dims {
    width: 960,
    height: 800,
};


pub struct WindowState {
    pub wb: Option<
        WindowBuilder>,
}

impl WindowState {
    pub fn new() -> WindowState {

        let wb = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(
                DIMS.width,
                DIMS.height,
            )).with_title("raytracer".to_string());

        WindowState {

            wb: Some(wb),
        }
    }
}

