use gfx_hal::{Backend, Device, Surface, SwapchainConfig, image as i, format, format::ChannelType};
use super::device::DeviceState;
use super::backend::BackendState;
use crate::window::DIMS;

use std::cell::RefCell;
use std::rc::Rc;
use gfx_hal::window::Extent2D;

pub struct SwapchainState<B: Backend> {
    pub swapchain: Option<B::Swapchain>,
    pub backbuffer: Vec<B::Image>,
    pub device: Rc<RefCell<DeviceState<B>>>,
    pub extent: i::Extent,
    pub format: format::Format,
}

impl<B: Backend> SwapchainState<B> {
    pub unsafe fn new(backend: &mut BackendState<B>, device: Rc<RefCell<DeviceState<B>>>) -> Self {

        let (caps, formats, _present_modes) = backend
            .surface
            .compatibility(&device.borrow().physical_device);
        println!("formats: {:?}", formats);
        let format = formats.map_or(format::Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });

        println!("Surface format: {:?}", format);

        let dims = Extent2D{
            width: DIMS.width,
            height: DIMS.height,
        };

        let mut swap_config = SwapchainConfig::from_caps(&caps, format, dims);

        swap_config.present_mode = gfx_hal::PresentMode::Immediate;
        swap_config.image_usage = i::Usage::STORAGE | i::Usage::COLOR_ATTACHMENT ;

        println!("Swap Config: {:?}", swap_config);
        let extent = swap_config.extent.to_extent();

        let (swapchain, backbuffer) = device
            .borrow()
            .device
            .create_swapchain(&mut backend.surface, swap_config, None)
            .expect("Can't create swapchain");


        let swapchain = SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: backbuffer,
            device,
            extent,
            format,
        };
        swapchain
    }

    pub unsafe fn number_of_images(&self) -> usize {
        let len = self.backbuffer.len();
        len
    }
}

impl<B: Backend> Drop for SwapchainState<B> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .borrow()
                .device
                .destroy_swapchain(self.swapchain.take().unwrap());
        }
    }
}
