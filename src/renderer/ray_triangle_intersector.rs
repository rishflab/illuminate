use gfx_hal::{Backend, Device, pso, image as i};

use gfx_hal::pso::DescriptorPool;

use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Read;
use std::path::Path;
use std::slice::Iter;
use crate::renderer::ENTRY_NAME;
use crate::renderer::device::DeviceState;

pub struct RayTriangleIntersector<B: Backend> {
    pub shader: B::ShaderModule,
    pub set_layout: B::DescriptorSetLayout,
    pub frame_set_layout: B::DescriptorSetLayout,
    pub layout: B::PipelineLayout,
    pub pipeline: B::ComputePipeline,
    pub desc_set: B::DescriptorSet,
    pub frame_desc_sets: Vec<B::DescriptorSet>,
    pub pool: B::DescriptorPool,
}

impl<B: Backend> RayTriangleIntersector<B> {

    pub unsafe fn write_frame_desc_sets(&self, device_state: Rc<RefCell<DeviceState<B>>>, image_views: &[B::ImageView]){

        let device = &device_state
            .borrow()
            .device;

        println!("writing frame desc_sets");
        println!("frame_desc_set length {:?}", self.frame_desc_sets.len());

        println!("view {:?}", image_views);

        self.frame_desc_sets
            .iter()
            .zip(image_views)
            .for_each(|(desc_set, view)| {

                println!("view {:?}", view);
                device
                    .write_descriptor_sets(vec![
                        pso::DescriptorSetWrite{
                            set: desc_set,
                            binding: 0,
                            array_offset: 0,
                            descriptors: Some(pso::Descriptor::Image(view, i::Layout::Present)),
                        }
                    ]);

            });

    }

    pub unsafe fn write_desc_set(&self, device_state: Rc<RefCell<DeviceState<B>>>,
                                 ray_buffer: &B::Buffer, vertex_buffer: &B::Buffer,
                                 index_buffer: &B::Buffer, camera_buffer: &B::Buffer){

        device_state
            .borrow()
            .device
            .write_descriptor_sets(vec![
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(ray_buffer, None..None)),
                },
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(index_buffer, None..None)),
                },
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 2,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(vertex_buffer, None..None)),
                },
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 3,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(camera_buffer, None..None)),
                },
            ]);

    }

    pub unsafe fn new(device_state: Rc<RefCell<DeviceState<B>>>,) -> Self {

        let device = &device_state
            .borrow()
            .device;

        let shader = {
            let path = Path::new("shaders").join("triangle_intersection.comp");
            let glsl = fs::read_to_string(path.as_path()).unwrap();
            let spirv: Vec<u8> = glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Compute)
                .expect("Could not compile shader")
                .bytes()
                .map(|b| b.unwrap())
                .collect();
            device.create_shader_module(&spirv).expect("Could not load shader module")
        };

        let frame_set_layout = device.create_descriptor_set_layout(
            &[
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::StorageImage,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
            ],
            &[],
        ).expect("Camera ray set layout creation failed");

        let set_layout = device.create_descriptor_set_layout(
            &[
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 2,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 3,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
            ],
            &[],
        ).expect("Camera ray set layout creation failed");

        let mut pool = device.create_descriptor_pool(
            6,
            &[
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 4,
                },
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::StorageImage,
                    count: 2,
                },
            ],
            pso::DescriptorPoolCreateFlags::empty(),
        ).expect("Camera ray descriptor pool creation failed");;


        let desc_set = pool.allocate_set(&set_layout).expect("Camera ray set allocation failed");

        let frame_desc_sets : Vec<B::DescriptorSet> = vec![0, 1]
            .iter()
            .map(|_|{
                pool.allocate_set(&frame_set_layout).expect("Camera ray set allocation failed")
           })
            .collect();

        let layout = device.create_pipeline_layout(vec![&frame_set_layout, &set_layout], &[])
            .expect("Camera ray pipeline layout creation failed");

//        let layout = device.create_pipeline_layout(vec![&frame_set_layout], &[])
//            .expect("Camera ray pipeline layout creation failed");

        let mut pipeline = {
            let shader_entry = pso::EntryPoint {
                entry: ENTRY_NAME,
                module: &shader,
                specialization: pso::Specialization::default(),
            };

            let pipeline_desc = pso::ComputePipelineDesc::new(
                shader_entry,
                &layout,
            );
            device.create_compute_pipeline(&pipeline_desc, None).expect("Could not create ray triangle intersector pipeline")
        };

        RayTriangleIntersector {
            shader,
            set_layout,
            frame_set_layout,
            pool,
            layout,
            desc_set,
            frame_desc_sets,
            pipeline,
        }
    }

}