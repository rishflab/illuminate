use super::DIMS;

use std::string::ToString;
use winit;

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

