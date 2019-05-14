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


use gfx_hal::{Backend, buffer::Usage, format, image, CommandPool};

use gfx_hal::{Device, pso, buffer as b};
use gfx_hal::window::Swapchain;


use gfx_hal::{pso::DescriptorSetLayoutBinding, DescriptorPool, pool, Compute, command, Submission,  QueueFamily};
use gfx_hal::format::Swizzle;


use std::cell::RefCell;
use std::rc::Rc;
use std::iter;
use crate::renderer::buffer::BufferState;
use crate::renderer::descriptor::DescriptorState;

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
    pub descriptor: DescriptorState<B>,
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


        let mut framebuffer = FramebufferState::new(
            Rc::clone(&device),
            &mut swapchain,
        );

        let descriptor = DescriptorState{
            descriptor_sets: framebuffer.write_descriptor_sets(Rc::clone(&device), desc_layout.get_layout(), desc_pool),
        };


        println!("created framebuffer");


        RendererState{
            swapchain: Some(swapchain),
            device: device,
            backend: backend,
            window: window,
            pipeline: pipeline,
            framebuffer: framebuffer,
            descriptor: descriptor,
        }

    }

    pub fn mainloop(&mut self) {

        let mut running = true;

        while running {
            {
                self.window.events_loop.poll_events(|event| {
                    if let winit::Event::WindowEvent { event, .. } = event {
                        #[allow(unused_variables)]
                            match event {
                            winit::WindowEvent::KeyboardInput {
                                input:
                                winit::KeyboardInput {
                                    virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            }
                            | winit::WindowEvent::CloseRequested => running = false,
                            _ => (),
                        }
                    }
                });

                let sem_index = self.framebuffer.next_acq_pre_pair_index();

                let frame: gfx_hal::SwapImageIndex = unsafe {
                    let (acquire_semaphore, _) = self
                        .framebuffer
                        .get_frame_data(None, Some(sem_index))
                        .1
                        .unwrap();

                    match self
                        .swapchain
                        .as_mut()
                        .unwrap()
                        .swapchain
                        .as_mut()
                        .unwrap()
                        .acquire_image(!0, Some(acquire_semaphore), None)
                        {
                            Ok((i, _)) => i,
                            Err(_) => {
                                continue
                            }
                        }
                };

                let (fid, sid) = self
                    .framebuffer
                    .get_frame_data(Some(frame as usize), Some(sem_index));

                let (framebuffer_fence, command_pool) = fid.unwrap();
                let (image_acquired, image_present) = sid.unwrap();

                unsafe {
                    self.device
                        .borrow()
                        .device
                        .wait_for_fence(framebuffer_fence, !0)
                        .unwrap();
                    self.device
                        .borrow()
                        .device
                        .reset_fence(framebuffer_fence)
                        .unwrap();

                    command_pool.reset();


                    let mut cmd_buffer = command_pool.acquire_command_buffer::<command::OneShot>();

                    cmd_buffer.begin();
                    cmd_buffer.bind_compute_pipeline(self.pipeline.pipeline.as_ref().unwrap());
                    cmd_buffer.bind_compute_descriptor_sets(self.pipeline.pipeline_layout.as_ref().unwrap(), 0, Some(&self.descriptor.descriptor_sets[frame as usize]), &[]);
                    cmd_buffer.dispatch([800, 800, 1]);
                    cmd_buffer.finish();


                    let submission = Submission {
                        command_buffers: iter::once(&cmd_buffer),
                        wait_semaphores: iter::once((&*image_acquired, pso::PipelineStage::BOTTOM_OF_PIPE)),
                        signal_semaphores: iter::once(&*image_present),
                    };

                    self.device.borrow_mut().queues.queues[0]
                        .submit(submission, Some(framebuffer_fence));

                    // present frame
                    if let Err(_) = self
                        .swapchain
                        .as_ref()
                        .unwrap()
                        .swapchain
                        .as_ref()
                        .unwrap()
                        .present(
                            &mut self.device.borrow_mut().queues.queues[0],
                            frame,
                            Some(&*image_present),
                        )
                    {
                        continue;
                    }
                }


            }
        }
    }
}