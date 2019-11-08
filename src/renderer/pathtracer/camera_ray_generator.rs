use gfx_hal::{Backend, Device, pso};
use gfx_hal::pso::DescriptorPool;
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Read;
use std::path::Path;
use crate::renderer::ENTRY_NAME;
use crate::renderer::core::device::DeviceState;

pub struct CameraRayGenerator<B: Backend> {
    pub shader: B::ShaderModule,
    pub set_layout: B::DescriptorSetLayout,
    pub layout: B::PipelineLayout,
    pub pipeline: B::ComputePipeline,
    pub desc_set: B::DescriptorSet,
    pub pool: B::DescriptorPool,
}

impl<B: Backend> CameraRayGenerator<B> {


    pub unsafe fn write_desc_set(&self, device_state: Rc<RefCell<DeviceState<B>>>,
                                 camera_buffer: &B::Buffer, ray_buffer: &B::Buffer,
                                 resolution_buffer: &B::Buffer){

        device_state
            .borrow()
            .device
            .write_descriptor_sets(vec![
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(camera_buffer, None..None)),
                },
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(ray_buffer, None..None)),
                },
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 2,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(resolution_buffer, None..None)),
                },
            ]);

    }

    pub unsafe fn new(device_state: Rc<RefCell<DeviceState<B>>>) -> Self {

        let device = &device_state
            .borrow()
            .device;

        let shader = {
            let path = Path::new("shaders").join("camera_rays.comp");
            let glsl = fs::read_to_string(path.as_path()).unwrap();
            let spirv: Vec<u8> = glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Compute)
                .expect("Could not compile shader")
                .bytes()
                .map(|b| b.unwrap())
                .collect();
            device.create_shader_module(&spirv).expect("Could not load shader module")
        };

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
            ],
            &[],
        ).expect("Camera ray set layout creation failed");;

        let mut pool = device.create_descriptor_pool(
            3,
            &[
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                },
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                },
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                },
            ],
            pso::DescriptorPoolCreateFlags::empty(),
        ).expect("Camera ray descriptor pool creation failed");;


        let desc_set = pool.allocate_set(&set_layout).expect("Camera ray set allocation failed");


        let layout = device.create_pipeline_layout(Some(&set_layout), &[])
            .expect("Camera ray pipeline layout creation failed");

        let pipeline = {
            let shader_entry = pso::EntryPoint {
                entry: ENTRY_NAME,
                module: &shader,
                specialization: pso::Specialization::default(),
            };

            let pipeline_desc = pso::ComputePipelineDesc::new(
                shader_entry,
                &layout,
            );

            device.create_compute_pipeline(&pipeline_desc, None).expect("Could not create camera ray pipeline")
        };

        CameraRayGenerator {
            shader,
            set_layout,
            pool,
            layout,
            desc_set,
            pipeline,
        }
    }

}