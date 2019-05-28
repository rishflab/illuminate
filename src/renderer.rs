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
    pub camera: Vec<Uniform<B>>,
    pub indices: Vec<Uniform<B>>,
    pub vertices: Vec<Uniform<B>>,
}

impl<B: Backend> RendererState<B> {

    pub unsafe fn new(mut backend: BackendState<B>, window: WindowState, scene: &Scene) -> Self {

        println!("creating render state");

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));


        let (desc_layout, camera_desc_layout, indices_desc_layout, vertices_desc_layout) = {

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

            let camera_desc = DescSetLayout::new(
                Rc::clone(&device),
                vec![pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                }],
            );

            let indices_desc = DescSetLayout::new(
                Rc::clone(&device),
                vec![pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                }],
            );

            let vertices_desc = DescSetLayout::new(
                Rc::clone(&device),
                vec![pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                }],
            );


            (desc_layout, camera_desc, indices_desc, vertices_desc)
        };

        let mut desc_pool = device
            .borrow()
            .device
            .create_descriptor_pool(
                8, // # of sets
                &[
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::StorageImage,
                        count: 2,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::UniformBuffer,
                        count: 2,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::UniformBuffer,
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

        let (descriptor, camera_desc, indices_desc, vertices_desc) = {
            let descriptor = DescriptorState {
                descriptor_sets: framebuffer.write_descriptor_sets(Rc::clone(&device), desc_layout.get_layout(), &mut desc_pool),
            };

            let camera_desc = (0..2).map(|_|{
                let desc = camera_desc_layout.create_desc_set(&mut desc_pool);
                desc
            }).collect::<Vec<_>>();

            let indices_desc = (0..2).map(|_|{
                let desc = indices_desc_layout.create_desc_set(&mut desc_pool);
                desc
            }).collect::<Vec<_>>();

            let vertices_desc = (0..2).map(|_|{
                let desc = vertices_desc_layout.create_desc_set(&mut desc_pool);
                desc
            }).collect::<Vec<_>>();

            (descriptor, camera_desc, indices_desc, vertices_desc)
        };

        let pipeline = PipelineState::new(
            vec![desc_layout.get_layout(), camera_desc_layout.get_layout(), indices_desc_layout.get_layout(), vertices_desc_layout.get_layout()],
            Rc::clone(&device),
        );

        let data = scene.camera_data();

        println!("view mat: {:?}", data);

        println!("Memory types: {:?}", backend.adapter.memory_types);

        let camera = camera_desc.into_iter().map(|d|{

            let uniform = Uniform::new(
                Rc::clone(&device),
                &backend.adapter.memory_types,
                &data,
                d,
                0,
            );
            uniform
        }).collect::<Vec<_>>();

        let indices = indices_desc.into_iter().map(|d|{

            let uniform = Uniform::new(
                Rc::clone(&device),
                &backend.adapter.memory_types,
                &scene.mesh_data.indices,
                d,
                0,
            );
            uniform
        }).collect::<Vec<_>>();

        let vertices = vertices_desc.into_iter().map(|d|{

            let uniform = Uniform::new(
                Rc::clone(&device),
                &backend.adapter.memory_types,
                &scene.mesh_data.positions,
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
            camera: camera,
            indices: indices,
            vertices: vertices,
        }

    }

    pub fn render(&mut self, scene: &Scene) {


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

        let data = scene.camera_data();

        //self.uniform[frame as usize].buffer.take().unwrap().update_data(0, &data);

        self.camera[frame as usize]
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
                    self.camera[frame as usize].desc.as_ref().unwrap().set.as_ref().unwrap(),
                    self.indices[frame as usize].desc.as_ref().unwrap().set.as_ref().unwrap(),
                    self.vertices[frame as usize].desc.as_ref().unwrap().set.as_ref().unwrap(),
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