use gfx_hal::{Backend, Device, Surface, SwapchainConfig, image, format, format::ChannelType};
use super::device::DeviceState;
use super::backend::BackendState;
use super::window::DIMS;

use std::cell::RefCell;
use std::rc::Rc;

pub struct SwapchainState<B: Backend> {
    pub swapchain: Option<B::Swapchain>,
    pub backbuffer: Option<Vec<B::Image>>,
    pub device: Rc<RefCell<DeviceState<B>>>,
    pub extent: image::Extent,
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
        let swap_config = SwapchainConfig::from_caps(&caps, format, DIMS);

        println!("Swap Config: {:?}", swap_config);
        let extent = swap_config.extent.to_extent();

        let (swapchain, backbuffer) = device
            .borrow()
            .device
            .create_swapchain(&mut backend.surface, swap_config, None)
            .expect("Can't create swapchain");


        let swapchain = SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: Some(backbuffer),
            device,
            extent,
            format,
        };
        swapchain
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
