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
pub mod uniform;
pub mod scene;

extern crate nalgebra_glm as glm;

use self::window::WindowState;
use self::backend::BackendState;
use self::device::DeviceState;
use self::swapchain::SwapchainState;
use self::pipeline::PipelineState;
use self::descriptor::DescSetLayout;
use self::framebuffer::FramebufferState;
use self::uniform::Uniform;
use self::scene::Scene;

use gfx_hal::{Backend, format, image};

use gfx_hal::{Device, pso};
use gfx_hal::window::Swapchain;


use gfx_hal::{command, Submission};

use std::cell::RefCell;
use std::rc::Rc;
use std::iter;
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
    pub uniform: Vec<Uniform<B>>,
}

impl<B: Backend> RendererState<B> {

    pub unsafe fn new(mut backend: BackendState<B>, window: WindowState) -> Self {

        println!("creating render state");

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));


        let (desc_layout, uniform_desc_layout) = {

            let desc_layout = DescSetLayout::new(
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

            let uniform_desc = DescSetLayout::new(
                Rc::clone(&device),
                vec![pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                }],
            );


            (desc_layout, uniform_desc)
        };


        let mut desc_pool = device
            .borrow()
            .device
            .create_descriptor_pool(
                4, // # of sets
                &[
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::StorageImage,
                        count: 2,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::UniformBuffer,
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

        println!("created framebuffer");

        let (descriptor, uniform_desc) = {
            let descriptor = DescriptorState {
                descriptor_sets: framebuffer.write_descriptor_sets(Rc::clone(&device), desc_layout.get_layout(), &mut desc_pool),
            };

            let uniform_desc = (0..2).map(|_|{
                let desc = uniform_desc_layout.create_desc_set(&mut desc_pool);
                desc
            }).collect::<Vec<_>>();

            (descriptor, uniform_desc)
        };

        let pipeline = PipelineState::new(
            vec![desc_layout.get_layout(), uniform_desc_layout.get_layout()],
            Rc::clone(&device),
        );


        let view = glm::look_at(
            &glm::vec3(1.0,2.0,7.0), // Camera is at (4,3,3), in World Space
            &glm::vec3(0.0,0.0,0.0), // and looks at the origin
            &glm::vec3(0.0,1.0,0.0)  // Head is up (set to 0,-1,0 to look upside-down)
        );

        let view = glm::inverse(&view);

        let view_vec: Vec<f32> = view.data.to_vec();
        let mut data = view_vec.clone();

        let model = glm::translation(&glm::vec3(0.0, 0.0, 0.0));

        let mut model_vec: Vec<f32> = model.as_slice().to_vec();

        data.append(&mut model_vec);

        let color = glm::vec4(0.0, 0.0, 0.0, 0.0);

        let mut color_vec: Vec<f32> = color.as_slice().to_vec();

        data.append(&mut color_vec);

        println!("view mat: {:?}", data);

        println!("Memory types: {:?}", backend.adapter.memory_types);

        let uniforms = uniform_desc.into_iter().map(|d|{

            let uniform = Uniform::new(
                Rc::clone(&device),
                &backend.adapter.memory_types,
                &data,
                d,
                0,
            );
            uniform
        }).collect::<Vec<_>>();


        RendererState{
            swapchain: Some(swapchain),
            device: device,
            backend: backend,
            window: window,
            pipeline: pipeline,
            framebuffer: framebuffer,
            descriptor: descriptor,
            uniform: uniforms,
        }

    }

    pub fn render(&mut self, scene: &Scene) {

        let uniform = &mut self.uniform;

        let view_vec: Vec<f32> = scene.camera.data.to_vec();

        let mut data = view_vec.clone();

        let model = scene.cube_model_mat();

        let mut model_vec: Vec<f32> = model.as_slice().to_vec();

        data.append(&mut model_vec);

        let color = scene.color;

        let mut color_vec: Vec<f32> = color.as_slice().to_vec();

        data.append(&mut color_vec);

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
                        panic!("couldnt acquire swapchain image")
                    }
                }
        };

        //self.uniform[frame as usize].buffer.take().unwrap().update_data(0, &data);

        uniform[frame as usize]
            .buffer
            .as_mut()
            .unwrap()
            .update_data(0, &data);

        let (fid, sid) = self
            .framebuffer
            .get_frame_data(Some(frame as usize), Some(sem_index));

        let (framebuffer_fence, command_pool) = fid.unwrap();
        let (image_acquired, image_present) = sid.unwrap();

        //println!("{:?}", image_acquired);

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
            cmd_buffer.bind_compute_descriptor_sets(
                self.pipeline.pipeline_layout.as_ref().unwrap(),
                0,
                vec!(
                    &self.descriptor.descriptor_sets[frame as usize],
                    self.uniform[frame as usize].desc.as_ref().unwrap().set.as_ref().unwrap(),
                ),
                &[]
            );
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
                panic!("couldnt present swapchain image")
            }
        }
    }



}