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
pub mod scene;
pub mod camera_rays;

extern crate nalgebra_glm as glm;

use self::window::WindowState;
use self::backend::BackendState;
use self::device::DeviceState;
use self::swapchain::SwapchainState;
use self::pipeline::PipelineState;
use self::framebuffer::FramebufferState;
use self::buffer::BufferState;
use self::scene::Scene;
use self::descriptor::DescSetLayout;

use gfx_hal::{Backend, format, image};

use gfx_hal::{Device, pso};
use gfx_hal::window::Swapchain;

use gfx_hal::{command, Submission};

use std::cell::RefCell;
use std::rc::Rc;
use std::iter;
use std::path::Path;

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
    pub frame_descriptors: Vec<B::DescriptorSet>,
    pub camera_descriptor: B::DescriptorSet,
    pub index_descriptor: B::DescriptorSet,
    pub vertex_descriptor: B::DescriptorSet,
    pub camera_buffer: BufferState<B>,
    pub index_buffer: BufferState<B>,
    pub vertex_buffer: BufferState<B>,
}

impl<B: Backend> RendererState<B> {

    pub unsafe fn new(mut backend: BackendState<B>, window: WindowState, scene: &Scene) -> Self {

        println!("creating render state");

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));


        let (frame_desc_layout, camera_desc_layout,
            indices_desc_layout, vertices_desc_layout) = {

            let frame_desc_layout = DescSetLayout::new(
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


            (frame_desc_layout, camera_desc, indices_desc, vertices_desc)
        };

        let mut swapchain = SwapchainState::new(&mut backend, Rc::clone(&device));
        println!("created swap chain");
        let number_of_images = swapchain.number_of_images();
        //let number_of_images = 1;
        println!("backbuffer size: {:?}", number_of_images);

        let mut desc_pool = device
            .borrow()
            .device
            .create_descriptor_pool(
                5,
                &[
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::StorageImage,
                        count: number_of_images,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::UniformBuffer,
                        count: 1,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::UniformBuffer,
                        count: 1,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::UniformBuffer,
                        count: 1,
                    },
                ],
                pso::DescriptorPoolCreateFlags::empty(),
            )
            .expect("Could not create descriptor pool");

        println!("created desc pool");


        let mut framebuffer = FramebufferState::new(
            Rc::clone(&device),
            &mut swapchain,
        );

        println!("created framebuffer");

        let camera_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            &scene.camera_data(),
        );

        let index_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            &scene.mesh_data.indices,
        );

        let vertex_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            &scene.mesh_data.positions,
        );

        let frame_descriptors = framebuffer.write_descriptor_sets(Rc::clone(&device), frame_desc_layout.get_layout(), &mut desc_pool);

        let (camera_descriptor, index_descriptor, vertex_descriptor) = {

            let camera_desc = camera_desc_layout.create_desc_set(&mut desc_pool, &camera_buffer.get_buffer());

            let indices_desc = indices_desc_layout.create_desc_set(&mut desc_pool, &index_buffer.get_buffer());

            let vertices_desc = vertices_desc_layout.create_desc_set(&mut desc_pool, &vertex_buffer.get_buffer());

            (camera_desc, indices_desc, vertices_desc)
        };

        let pipeline = PipelineState::new(
            vec![frame_desc_layout.get_layout(), camera_desc_layout.get_layout(), indices_desc_layout.get_layout(), vertices_desc_layout.get_layout()],
            Rc::clone(&device),
            Path::new("shaders").join("raytracer.comp").as_path(),
        );

        let data = scene.camera_data();

        println!("view mat: {:?}", data);

        println!("Memory types: {:?}", backend.adapter.memory_types);


        RendererState{
            swapchain: Some(swapchain),
            device: device,
            backend: backend,
            window: window,
            pipeline: pipeline,
            framebuffer: framebuffer,
            frame_descriptors,
            camera_buffer,
            camera_descriptor,
            index_buffer,
            index_descriptor,
            vertex_buffer,
            vertex_descriptor,
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
        self.camera_buffer
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
                    &self.frame_descriptors[frame as usize],
                    &self.camera_descriptor,
                    &self.index_descriptor,
                    &self.vertex_descriptor,
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