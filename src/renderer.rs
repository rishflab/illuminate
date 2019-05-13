//extern crate env_logger;

//extern crate gfx_backend_vulkan as back;

pub mod window;
pub mod device;
pub mod backend;
pub mod swapchain;
pub mod pipeline;
pub mod buffer;
pub mod descriptor;
pub mod framebuffer;

use self::window::WindowState;
use self::backend::BackendState;
use self::device::DeviceState;
use self::swapchain::SwapchainState;
use self::pipeline::PipelineState;
use self::descriptor::DescSetLayout;
use self::framebuffer::FramebufferState;


use gfx_hal::{Backend, buffer::Usage, format, image};

use gfx_hal::{Device, pso, buffer as b};


use gfx_hal::{pso::DescriptorSetLayoutBinding, DescriptorPool};
use gfx_hal::format::Swizzle;


use std::cell::RefCell;
use std::rc::Rc;
use crate::renderer::buffer::BufferState;

const ENTRY_NAME: &str = "main";

const COLOR_RANGE: image::SubresourceRange = image::SubresourceRange {
    aspects: format::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

pub struct RendererState<B: Backend> {
    pub swapchain: Option<SwapchainState<B>>,
    pub device: Rc<RefCell<DeviceState<B>>>,
    pub backend: BackendState<B>,
    pub window: WindowState,
    pub pipeline: PipelineState<B>,
    pub framebuffer: FramebufferState<B>,
}

impl<B: Backend> RendererState<B> {

    pub unsafe fn new(mut backend: BackendState<B>, window: WindowState) -> Self {

        println!("creating render state");

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));


        let (desc_layout, pipeline) = {
            let mut desc_layout = DescSetLayout::new(
                Rc::clone(&device),
                vec![
                    pso::DescriptorSetLayoutBinding {
                        binding: 0,
                        ty: pso::DescriptorType::StorageImage,
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::COMPUTE,
                        immutable_samplers: false,
                    },
                ],
            );
            let pipeline = PipelineState::new(
                vec![desc_layout.get_layout()],
                Rc::clone(&device),
            );
            (desc_layout, pipeline)
        };


        let mut desc_pool = device
            .borrow()
            .device
            .create_descriptor_pool(
                2, // # of sets
                &[
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::StorageImage,
                        count: 2,
                    },
                ],
                pso::DescriptorPoolCreateFlags::empty(),
            )
            .expect("Could not create descriptor pool");

        println!("created desc pool");


        let mut swapchain = SwapchainState::new(&mut backend, Rc::clone(&device));

        println!("created swap chain");


        let framebuffer = FramebufferState::new(
            Rc::clone(&device),
            &desc_layout.layout.unwrap(),
            desc_pool,
            &mut swapchain,
        );


        println!("created framebuffer");


        RendererState{
            swapchain: Some(swapchain),
            device: device,
            backend: backend,
            window: window,
            pipeline: pipeline,
            framebuffer: framebuffer,
        }

    }

    pub fn mainloop(&mut self){

    }
}