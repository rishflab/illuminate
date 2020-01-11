use gfx_hal::{
    Backend, image as i, format, format::ChannelType, pso, pso::DescriptorPool, prelude::*,
    window::SwapchainConfig,
};
use super::device::DeviceState;
use super::backend::BackendState;
use crate::window::DIMS;
use crate::renderer::COLOR_RANGE;

use std::cell::RefCell;
use std::rc::Rc;
use gfx_hal::window::Extent2D;

pub struct SwapchainState<B: Backend> {
    pub swapchain: B::Swapchain,
    pub backbuffer: Vec<B::Image>,
    pub image_views: Vec<B::ImageView>,
    pub device: Rc<RefCell<DeviceState<B>>>,
    pub extent: i::Extent,
    pub format: format::Format,
}

impl<B: Backend> SwapchainState<B> {
    pub unsafe fn new(backend: &mut BackendState<B>, device: Rc<RefCell<DeviceState<B>>>) -> Self {
        let caps = backend.surface.capabilities(&device.borrow().physical_device);
        let formats = backend.surface.supported_formats(&device.borrow().physical_device);
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
        //swap_config.present_mode = gfx_hal::window::PresentMode::Immediate;
        swap_config.image_usage = i::Usage::STORAGE | i::Usage::COLOR_ATTACHMENT ;
        println!("Swap Config: {:?}", swap_config);

        let extent = swap_config.extent.to_extent();

        let (swapchain, backbuffer) = device
            .borrow()
            .device
            .create_swapchain(&mut backend.surface, swap_config, None)
            .expect("Can't create swapchain");

        let image_views: Vec<B::ImageView>  = {
            backbuffer
                .iter()
                .map(|image| {
                    println!("creating image view");
                    let view = device
                        .borrow()
                        .device
                        .create_image_view(
                            &image,
                            i::ViewKind::D2,
                            format,
                            format::Swizzle::NO,
                            COLOR_RANGE.clone(),
                        )
                        .unwrap();

                    println!("{:?}", view);
                    view
                })
                .collect()
        };


        let swapchain = SwapchainState {
            swapchain,
            backbuffer,
            image_views,
            device,
            extent,
            format,
        };
        swapchain
    }

    pub unsafe fn write_descriptor_sets(
        &mut self,
        device: Rc<RefCell<DeviceState<B>>>,
        desc_layout: &B::DescriptorSetLayout,
        desc_pool: &mut B::DescriptorPool,
    ) -> Vec<B::DescriptorSet> {

        self.image_views
            .iter()
            .map(|view| {
                let desc_set = desc_pool.allocate_set(desc_layout).unwrap();

                device
                    .borrow()
                    .device
                    .write_descriptor_sets(Some(
                        pso::DescriptorSetWrite {
                            set: &desc_set,
                            binding: 0,
                            array_offset: 0,
                            descriptors: Some(pso::Descriptor::Image(view, i::Layout::Present)),
                        }
                    ));


                desc_set
            })
            .collect::<Vec<_>>()

    }

    pub unsafe fn number_of_images(&self) -> usize {
        let len = self.backbuffer.len();
        len
    }

    pub fn get_image_views(&self) -> &[B::ImageView] {
        &self.image_views
    }
}
//
//impl<B: Backend> Drop for SwapchainState<B> {
//    fn drop(&mut self) {
//        let device = &self.device.borrow().device;
//
//        unsafe {
//
//            for view in self.image_views.take().unwrap() {
//                device.destroy_image_view(view);
//            }
//
//            for image in self.backbuffer.take().unwrap() {
//                device.destroy_image(image);
//            }
//
//            self.destroy_swapchain(self.swapchain.take().unwrap());
//
//        }
//    }
//}
