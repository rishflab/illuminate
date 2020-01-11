pub mod ray_triangle_intersector;
pub mod camera_ray_generator;
pub mod types;
pub mod accumulator;
pub mod vertex_skinner;
pub mod aabb_calculator;

use crate::window::WindowState;
use crate::renderer::core::backend::BackendState;
use crate::renderer::core::device::DeviceState;
use crate::renderer::core::swapchain::SwapchainState;
use crate::renderer::core::command::CommandState;
use crate::renderer::core::buffer::BufferState;
use crate::scene::{Scene};
use self::camera_ray_generator::CameraRayGenerator;
use self::ray_triangle_intersector::RayTriangleIntersector;
use self::accumulator::Accumulator;
use self::vertex_skinner::VertexSkinner;
use self::aabb_calculator::AabbCalculator;


use self::types::Ray;
use self::types::Intersection;
use crate::window::DIMS;
use crate::renderer::RAY_SAMPLES;
use crate::renderer::WORK_GROUP_SIZE;

use gfx_hal::{prelude::*, Backend, command, pso, memory, buffer, pool, queue::Submission, window::SwapImageIndex};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Pathtracer<B: Backend> {
    pub swapchain: SwapchainState<B>,
    pub device: Rc<RefCell<DeviceState<B>>>,
    pub backend: BackendState<B>,
    pub command: CommandState<B>,
    pub camera_ray_generator: CameraRayGenerator<B>,
    pub ray_triangle_intersector: RayTriangleIntersector<B>,
    pub vertex_skinner: VertexSkinner<B>,
    pub aabb_calculator: AabbCalculator<B>,
    pub accumulator: Accumulator<B>,
    pub camera_buffer: BufferState<B>,
    pub vertex_buffer: BufferState<B>,
    pub triangle_buffer: BufferState<B>,
    pub index_buffer: BufferState<B>,
    pub primary_ray_buffer: BufferState<B>,
    pub primary_intersection_buffer: BufferState<B>,
    pub bounce_ray_buffer: BufferState<B>,
    pub bounce_intersection_buffer: BufferState<B>,
    pub aabb_buffer: BufferState<B>,
    pub model_buffer: BufferState<B>,
    pub light_buffer: BufferState<B>,
    pub resolution_buffer: BufferState<B>,
}

impl<B: Backend> Pathtracer<B> {

//    pub unsafe fn new(mut backend: BackendState<B>, window: WindowState, scene: &Scene) -> Self {
    pub unsafe fn new(mut backend: BackendState<B>, scene: &Scene) -> Self {

        println!("creating render state");

        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));

        let swapchain = SwapchainState::new(&mut backend, Rc::clone(&device));
        println!("created swap chain");

        let number_of_images = swapchain.number_of_images();
        println!("backbuffer size: {:?}", number_of_images);

        let command = CommandState::new(
            Rc::clone(&device),
            number_of_images
        );
        println!("created command buffer state");

        let camera_ray_generator = CameraRayGenerator::new(Rc::clone(&device));

        let ray_triangle_intersector = RayTriangleIntersector::new(Rc::clone(&device));

        let accumulator = Accumulator::new(Rc::clone(&device));

        let vertex_skinner = VertexSkinner::new(Rc::clone(&device));

        let aabb_calculator = AabbCalculator::new(Rc::clone(&device));

        println!("Created pipelines");

        println!("memory types: {:?}", &backend.adapter.memory_types);

        let camera_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::CPU_VISIBLE,
            buffer::Usage::STORAGE,
            &scene.view_matrix().data,
        );

        let resolution_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::CPU_VISIBLE,
            buffer::Usage::STORAGE,
            &glm::vec2(DIMS.width, DIMS.height).data,
        );

        let model_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::CPU_VISIBLE,
            buffer::Usage::STORAGE,
            &scene.model_matrices()
        );

        let light_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::CPU_VISIBLE | memory::Properties::DEVICE_LOCAL,
            buffer::Usage::STORAGE | buffer::Usage::TRANSFER_DST | buffer::Usage::TRANSFER_SRC,
            &scene.light_data(),
        );


        let primary_ray_buffer = BufferState::empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::DEVICE_LOCAL,
            buffer::Usage::STORAGE,
            (DIMS.width * DIMS.height * RAY_SAMPLES) as u64,
            Ray{
                origin: [0.0, 0.0, 0.0, 0.0],
                direction: [0.0, 0.0, 0.0, 0.0],
            }
        );

        let primary_intersection_buffer = BufferState::empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::DEVICE_LOCAL,
             buffer::Usage::STORAGE,
            (DIMS.width * DIMS.height * RAY_SAMPLES) as u64,
            Intersection{
                position: [0.0, 0.0, 0.0, 0.0],
                normal: [0.0, 0.0, 0.0, 0.0],
                edge: [0.0, 0.0, 0.0, 0.0],
                float: 0.0
            }
        );

        let bounce_ray_buffer = BufferState::empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::DEVICE_LOCAL,
            buffer::Usage::STORAGE,
            (DIMS.width * DIMS.height * RAY_SAMPLES) as u64,
            Ray{
                origin: [0.0, 0.0, 0.0, 0.0],
                direction: [0.0, 0.0, 0.0, 0.0],
            }
        );

        let bounce_intersection_buffer = BufferState::empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::DEVICE_LOCAL,
            buffer::Usage::STORAGE,
            (DIMS.width * DIMS.height * RAY_SAMPLES) as u64,
            Intersection{
                position: [0.0, 0.0, 0.0, 0.0],
                normal: [0.0, 0.0, 0.0, 0.0],
                edge: [0.0, 0.0, 0.0, 0.0],
                float: 0.0
            }
        );

        let index_buffer = BufferState::empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::DEVICE_LOCAL,
            buffer::Usage::TRANSFER_DST | buffer::Usage::STORAGE,
            scene.total_unique_indices() as u64,
            types::Index(0),
        );

        let vertex_buffer = BufferState::empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::DEVICE_LOCAL,
            buffer::Usage::TRANSFER_DST | buffer::Usage::STORAGE,
            scene.total_unique_vertices() as u64,
            types::Vertex([0.0, 0.0, 0.0, 0.0])
        );

        let triangle_buffer = BufferState::empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::DEVICE_LOCAL,
            buffer::Usage::STORAGE,
            scene.total_indices() as u64,
            types::Vertex([0.0, 0.0, 0.0, 0.0])

        );

        let aabb_buffer = BufferState::empty(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::DEVICE_LOCAL,
            buffer::Usage::STORAGE,
            scene.mesh_instances.len() as u64,
            types::Aabb{min: [0.0, 0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0, 0.0]}
        );

        let staging_index_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::CPU_VISIBLE,
            buffer::Usage::TRANSFER_SRC,
            &scene.index_data(),
        );

        let staging_vertex_buffer = BufferState::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            memory::Properties::CPU_VISIBLE,
            buffer::Usage::TRANSFER_SRC,
            &scene.vertex_data(),
        );

        camera_ray_generator.write_desc_set(
            Rc::clone(&device),
            camera_buffer.get_buffer(),
            primary_ray_buffer.get_buffer(),
            resolution_buffer.get_buffer(),
        );

        vertex_skinner.write_desc_set(
            Rc::clone(&device),
            model_buffer.get_buffer(),
            vertex_buffer.get_buffer(),
            triangle_buffer.get_buffer(),
            index_buffer.get_buffer(),
        );

        aabb_calculator.write_desc_set(
            Rc::clone(&device),
            triangle_buffer.get_buffer(),
            aabb_buffer.get_buffer(),
        );

        ray_triangle_intersector.write_desc_set(
            Rc::clone(&device),
            triangle_buffer.get_buffer(),
            aabb_buffer.get_buffer(),
            primary_ray_buffer.get_buffer(),
            primary_intersection_buffer.get_buffer(),
            bounce_ray_buffer.get_buffer(),
            bounce_intersection_buffer.get_buffer(),
        );

        accumulator.write_desc_set(
            Rc::clone(&device),
            light_buffer.get_buffer(),
            resolution_buffer.get_buffer(),
            primary_intersection_buffer.get_buffer(),
            bounce_intersection_buffer.get_buffer(),
            triangle_buffer.get_buffer(),
            aabb_buffer.get_buffer(),
        );

        accumulator.write_frame_desc_sets(
            Rc::clone(&device),
            swapchain.get_image_views(),
        );

        let mut transfered_image_fence = device.borrow().device.create_fence(false)
            .expect("Can't create fence");

        let mut staging_pool = device
            .borrow()
            .device
            .create_command_pool(
                device.borrow().queues.family,
                pool::CommandPoolCreateFlags::empty(),
            )
            .expect("Can't create staging command pool");

        let mut cmd_buffer = staging_pool.allocate_one(command::Level::Primary);

        cmd_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);

        cmd_buffer.copy_buffer(
            &staging_index_buffer.get_buffer(),
            &index_buffer.get_buffer(),
            &[
                command::BufferCopy {
                    src: 0,
                    dst: 0,
                    size: staging_index_buffer.size,
                },
            ],
        );

        cmd_buffer.copy_buffer(
            &staging_vertex_buffer.get_buffer(),
            &vertex_buffer.get_buffer(),
            &[
                command::BufferCopy {
                    src: 0,
                    dst: 0,
                    size: staging_vertex_buffer.size,
                },
            ],
        );

        cmd_buffer.finish();

        device.borrow_mut().queues.queues[0]
            .submit_without_semaphores(&[cmd_buffer], Some(&mut transfered_image_fence));

        device
            .borrow()
            .device
            .destroy_command_pool(staging_pool);

        Pathtracer {
            swapchain: swapchain,
            device: device,
            backend: backend,
            command,
            camera_ray_generator,
            ray_triangle_intersector,
            accumulator,
            vertex_skinner,
            camera_buffer,
            index_buffer,
            vertex_buffer,
            triangle_buffer,
            primary_ray_buffer,
            primary_intersection_buffer,
            bounce_ray_buffer,
            bounce_intersection_buffer,
            aabb_calculator,
            aabb_buffer,
            model_buffer,
            light_buffer,
            resolution_buffer
        }
    }

    pub fn render(&mut self, scene: &Scene) {

        self.camera_buffer.update_data(0, &scene.view_matrix().data);
        self.light_buffer.update_data(0, &scene.light_data());
        self.model_buffer.update_data(0, &scene.model_matrices());

        // Use guaranteed unused acquire semaphore to get the index of the next frame we will render to
        // by using acquire_image
        let swap_image = unsafe {
            match self.swapchain.swapchain.acquire_image(!0, Some(&self.command.free_acquire_semaphore), None) {
                Ok((i, _)) => i as usize,
                Err(_) => {
                    panic!("Could not acquire swapchain image");
                }
            }
        };

        // Swap the acquire semaphore with the one previously associated with the image we are acquiring
        core::mem::swap(
            &mut self.command.free_acquire_semaphore,
            &mut self.command.image_acquire_semaphores[swap_image],
        );

        // Compute index into our resource ring buffers based on the frame number
        // and number of frames in flight. Pay close attention to where this index is needed
        // versus when the swapchain image index we got from acquire_image is needed.
        let frame_idx = self.command.frame % self.command.frames_in_flight;

        // Wait for the fence of the previous submission of this frame and reset it; ensures we are
        // submitting only up to maximum number of frames_in_flight if we are submitting faster than
        // the gpu can keep up with. This would also guarantee that any resources which need to be
        // updated with a CPU->GPU data copy are not in use by the GPU, so we can perform those updates.
        // In this case there are none to be done, however.
        unsafe {
            &self.device.borrow().device
                .wait_for_fence(&self.command.submission_complete_fences[frame_idx], !0)
                .expect("Failed to wait for fence");
            &self.device.borrow().device
                .reset_fence(&self.command.submission_complete_fences[frame_idx])
                .expect("Failed to reset fence");
            self.command.command_pools[frame_idx].reset(false);
        }

        // Rendering
        let cmd_buffer = &mut self.command.command_buffers[frame_idx];


        unsafe {

            cmd_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);

            cmd_buffer.bind_compute_pipeline(&self.camera_ray_generator.pipeline);
            cmd_buffer.bind_compute_descriptor_sets(
                &self.camera_ray_generator.layout,
                0,
                vec!(
                    &self.camera_ray_generator.desc_set,
                ),
                &[]
            );
            cmd_buffer.dispatch([(DIMS.width*DIMS.height)/(WORK_GROUP_SIZE*WORK_GROUP_SIZE), RAY_SAMPLES, 1]);


            cmd_buffer.bind_compute_pipeline(&self.vertex_skinner.pipeline);

            for view in scene.instance_views(){
                cmd_buffer.bind_compute_descriptor_sets(
                    &self.vertex_skinner.layout,
                    0,
                    vec!(
                        &self.vertex_skinner.desc_set
                    ),
                    &[]
                );
                cmd_buffer.push_compute_constants(
                    &self.vertex_skinner.layout,
                    0,
                    &[view.instance_id, view.start],
                );

                cmd_buffer.dispatch([view.length, 1, 1]);

                let t_barrier = memory::Barrier::Buffer{
                    states: buffer::Access::SHADER_WRITE..buffer::Access::SHADER_READ,
                    target: self.triangle_buffer.get_buffer(),
                    families: None,
                    range: Some(view.start as u64)..Some((view.start + view.length) as u64)
                    //range: None..None,
                };

                cmd_buffer.pipeline_barrier(
                    pso::PipelineStage::COMPUTE_SHADER..pso::PipelineStage::COMPUTE_SHADER,
                    memory::Dependencies::empty(),
                    &[t_barrier],
                );
            }

            let ray_barrier = memory::Barrier::Buffer{
                states: buffer::Access::SHADER_WRITE..buffer::Access::SHADER_READ,
                target: self.primary_ray_buffer.get_buffer(),
                families: None,
                range: None..None
            };

            cmd_buffer.pipeline_barrier(
                pso::PipelineStage::COMPUTE_SHADER..pso::PipelineStage::COMPUTE_SHADER,
                memory::Dependencies::empty(),
                &[ray_barrier],
            );

            cmd_buffer.bind_compute_pipeline(&self.aabb_calculator.pipeline);


            for view in scene.instance_views() {

                cmd_buffer.bind_compute_descriptor_sets(
                    &self.aabb_calculator.layout,
                    0,
                    vec!(
                        &self.aabb_calculator.desc_set
                    ),
                    &[]
                );
                cmd_buffer.push_compute_constants(
                    &self.aabb_calculator.layout,
                    0,
                    &[view.start, view.start + view.length, view.instance_id],
                );

                cmd_buffer.dispatch([scene.mesh_instances.len() as u32, 1, 1]);

                let aabb_barrier = memory::Barrier::Buffer{
                    states: buffer::Access::SHADER_READ..buffer::Access::SHADER_READ,
                    target: self.aabb_buffer.get_buffer(),
                    families: None,
                    range: None..None
                };

                cmd_buffer.pipeline_barrier(
                    pso::PipelineStage::COMPUTE_SHADER..pso::PipelineStage::COMPUTE_SHADER,
                    memory::Dependencies::empty(),
                    &[aabb_barrier],
                );
            }

            cmd_buffer.bind_compute_pipeline(&self.ray_triangle_intersector.pipeline);
            cmd_buffer.bind_compute_descriptor_sets(
                &self.ray_triangle_intersector.layout,
                0,
                vec!(
                    &self.ray_triangle_intersector.desc_set
                ),
                &[]
            );

            cmd_buffer.dispatch([(DIMS.width*DIMS.height*RAY_SAMPLES)/(WORK_GROUP_SIZE*WORK_GROUP_SIZE),1 , 1]);

            let intersection_barrier = memory::Barrier::Buffer{
                states: buffer::Access::SHADER_WRITE..buffer::Access::SHADER_READ,
                target: self.primary_intersection_buffer.get_buffer(),
                families: None,
                range: None..None
            };

            cmd_buffer.pipeline_barrier(
                pso::PipelineStage::COMPUTE_SHADER..pso::PipelineStage::COMPUTE_SHADER,
                memory::Dependencies::empty(),
                &[intersection_barrier],
            );

            cmd_buffer.bind_compute_pipeline(&self.accumulator.pipeline);
            cmd_buffer.bind_compute_descriptor_sets(
                &self.accumulator.layout,
                0,
                vec!(
                    &self.accumulator.frame_desc_sets[frame_idx],
                    &self.accumulator.desc_set,
                ),
                &[]
            );

            cmd_buffer.dispatch([(DIMS.width*DIMS.height)/(WORK_GROUP_SIZE*WORK_GROUP_SIZE),1 , 1]);

            cmd_buffer.finish();

            let submission = Submission {
                command_buffers: Some(&*cmd_buffer),
                wait_semaphores: Some((
                    &self.command.image_acquire_semaphores[swap_image],
                    pso::PipelineStage::BOTTOM_OF_PIPE,
                )),
                signal_semaphores: Some(&self.command.submission_complete_semaphores[frame_idx]),
            };

            self.device.borrow_mut().queues.queues[0]
                .submit(submission, Some(&self.command.submission_complete_fences[frame_idx]));

            // present frame
            self.swapchain.swapchain.present(
                &mut self.device.borrow_mut().queues.queues[0],
                swap_image as SwapImageIndex,
                Some(&self.command.submission_complete_semaphores[frame_idx]),
            );

        }
        // Increment our frame
        self.command.frame += 1;
    }
}
