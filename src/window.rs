use std::string::ToString;
use winit;

pub struct Dims {
    pub width: u32,
    pub height: u32,
}

pub const DIMS: Dims = Dims {
    width: 960,
    height: 800,
};


pub struct WindowState {
    pub wb: Option<winit::WindowBuilder>,
}

impl WindowState {
    pub fn new() -> WindowState {

        let wb = winit::WindowBuilder::new()
            .with_dimensions(winit::dpi::LogicalSize::new(
                DIMS.width as _,
                DIMS.height as _,
            )).with_title("raytracer".to_string());

        WindowState {

            wb: Some(wb),
        }
    }
}

