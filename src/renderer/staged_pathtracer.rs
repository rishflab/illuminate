pub mod ray_triangle_intersector;
pub mod camera_ray_generator;
pub mod types;
pub mod accumulator;

use crate::window::WindowState;
use crate::renderer::core::backend::BackendState;
use crate::renderer::core::device::DeviceState;
use crate::renderer::core::swapchain::SwapchainState;
use crate::renderer::core::pipeline::PipelineState;
use crate::renderer::core::framebuffer::FramebufferState;
use crate::renderer::core::buffer::BufferState;
use crate::renderer::scene::Scene;
use crate::renderer::core::descriptor::DescSetLayout;
use self::camera_ray_generator::CameraRayGenerator;
use self::ray_triangle_intersector::RayTriangleIntersector;
use self::accumulator::Accumulator;
use crate::renderer::Renderer;
use self::types::Ray;
use self::types::Intersection;
use crate::window::DIMS;

use gfx_hal::{Backend, Device, Submission, Swapchain, command, pso, format, image, memory, buffer as b};

use std::cell::RefCell;
use std::rc::Rc;
use std::iter;
use std::path::Path;

pub struct StagedPathtracer<B: Backend> {
    pub swapchain: Option<SwapchainState<B>>,
    pub device: Rc<RefCell<DeviceState<B>>>,
    pub backend: BackendState<B>,
    pub window: WindowState,
    pub framebuffer: FramebufferState<B>,
    pub camera_ray_generator: CameraRayGenerator<B>,
    pub ray_triangle_intersector: RayTriangleIntersector<B>,
    pub accumulator: Accumulator<B>,
    pub camera_buffer: BufferState<B>,
    pub ray_buffer: BufferState<B>,
    pub vertex_buffer: BufferState<B>,
    pub index_buffer: BufferState<B>,
    pub intersection_buffer: BufferState<B>,
}

impl<B: Backend> StagedPathtracer<B> {

    pub unsafe fn new(mut backend: BackendState<B>, window: WindowState, scene: &Scene) -> Self {

        println!("creating render state");

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));

        let mut swapchain = SwapchainState::new(&mut backend, Rc::clone(&device));
        println!("created swap chain");

        let number_of_images = swapchain.number_of_images();
        println!("backbuffer size: {:?}", number_of_images);

        let mut framebuffer = FramebufferState::new(
            Rc::clone(&device),
            &mut swapchain,
        );
        println!("created framebuffer");

        let camera_ray_generator = CameraRayGenerator::new(Rc::clone(&device));

        let ray_triangle_intersector = RayTriangleIntersector::new(Rc::clone(&device));

        let accumulator = Accumulator::new(Rc::clone(&device));

        let camera_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            &scene.camera_data(),
        );

        let ray_buffer = BufferState::new_empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            (DIMS.width * DIMS.height) as u64,
            Ray{
                origin: [0.0, 0.0, 0.0, 0.0],
                direction: [0.0, 0.0, 0.0, 0.0],
            }
        );

        let intersection_buffer = BufferState::new_empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            (DIMS.width * DIMS.height * 12) as u64,
            Intersection{
                color: [0.0, 0.0, 0.0, 0.0],
            }
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

        camera_ray_generator.write_desc_set(
            Rc::clone(&device),
            camera_buffer.get_buffer(),
            ray_buffer.get_buffer(),
        );

        ray_triangle_intersector.write_desc_set(
            Rc::clone(&device),
            ray_buffer.get_buffer(),
            vertex_buffer.get_buffer(),
            index_buffer.get_buffer(),
            camera_buffer.get_buffer(),
            intersection_buffer.get_buffer(),
        );

        accumulator.write_desc_set(
            Rc::clone(&device),
            intersection_buffer.get_buffer(),
        );

        accumulator.write_frame_desc_sets(
            Rc::clone(&device),
            framebuffer.get_image_views(),
        );


        StagedPathtracer {
            swapchain: Some(swapchain),
            device: device,
            backend: backend,
            window: window,
            framebuffer: framebuffer,
            camera_ray_generator,
            ray_triangle_intersector,
            accumulator,
            camera_buffer,
            ray_buffer,
            index_buffer,
            vertex_buffer,
            intersection_buffer,
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

        //println!("frame {:?}", frame);

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
            cmd_buffer.bind_compute_pipeline(&self.camera_ray_generator.pipeline);
            cmd_buffer.bind_compute_descriptor_sets(
                &self.camera_ray_generator.layout,
                0,
                vec!(
                    &self.camera_ray_generator.desc_set
                ),
                &[]
            );
            cmd_buffer.dispatch([DIMS.width, DIMS.height, 1]);
//
//            let ray_barrier = memory::Barrier::Buffer {
//                states: b::Access::SHADER_WRITE..b::Access::SHADER_READ,
//                target: self.ray_buffer.get_buffer(),
//                //families: Some(self.device.borrow().get_queue_family_id()..self.device.borrow().get_queue_family_id()),
//                families: None,
//                /// Range of the buffer the barrier applies to.
//                range: Some(0 as u64)..Some(self.ray_buffer.size as u64),
//                //range: None..None,
//            };


//            cmd_buffer.pipeline_barrier(
//                pso::PipelineStage::COMPUTE_SHADER..pso::PipelineStage::COMPUTE_SHADER,
//                memory::Dependencies::empty(),
//                &[ray_barrier],
//            );


            cmd_buffer.bind_compute_pipeline(&self.ray_triangle_intersector.pipeline);
            cmd_buffer.bind_compute_descriptor_sets(
                &self.ray_triangle_intersector.layout,
                0,
                vec!(
                    &self.ray_triangle_intersector.desc_set
                ),
                &[]
            );

            cmd_buffer.dispatch([DIMS.width, DIMS.height, 12]);



            cmd_buffer.bind_compute_pipeline(&self.accumulator.pipeline);
            cmd_buffer.bind_compute_descriptor_sets(
                &self.accumulator.layout,
                0,
                vec!(
                    &self.accumulator.frame_desc_sets[frame as usize],
                    &self.accumulator.desc_set
                ),
                &[]
            );

            cmd_buffer.dispatch([DIMS.width, DIMS.height, 1]);

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