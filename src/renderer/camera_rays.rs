use gfx_hal::{Backend, Device, pso};

use gfx_hal::pso::DescriptorPool;

use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Read;
use std::path::Path;
use crate::renderer::ENTRY_NAME;

pub struct CameraRays<B: Backend> {
    pub shader: B::ShaderModule,
    pub set_layout: B::DescriptorSetLayout,
    pub layout: B::PipelineLayout,
    pub pipeline: B::ComputePipeline,
    pub desc_sets: Vec<B::DescriptorSet>,
    pub pool: B::DescriptorPool,
}

impl<B: Backend> CameraRays<B> {

    pub unsafe fn new(device: &mut B::Device) -> Self {

        let backbuffer_size = 2;

        let shader = {
            let path = Path::new("shaders").join("camera_rays.comp");
            let glsl = fs::read_to_string(path.as_path()).unwrap();
            let spirv: Vec<u8> = glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Compute)
                .unwrap()
                .bytes()
                .map(|b| b.unwrap())
                .collect();
            device.create_shader_module(&spirv).expect("Could not load shader module")
        };

        let set_layout = device.create_descriptor_set_layout(
            vec![pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::StorageBuffer,
                count: 1,
                stage_flags: pso::ShaderStageFlags::COMPUTE,
                immutable_samplers: false,
            }],
            &[],
        ).expect("Camera ray set layout creation failed");;

        let mut pool = device.create_descriptor_pool(
            backbuffer_size * 1,
            &[
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::StorageBuffer,
                    count: backbuffer_size * 1,
                },
            ],
            pso::DescriptorPoolCreateFlags::empty(),
        ).expect("Camera ray descriptor pool creation failed");;


        let desc_sets = [0..backbuffer_size]
            .iter()
            .map(|_|{
                pool.allocate_set(&set_layout).expect("Camera ray set allocation failed")
            })
            .collect();

        let layout = device.create_pipeline_layout(Some(&set_layout), &[])
            .expect("Camera ray pipeline layout creation failed");

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

            device.create_compute_pipeline(&pipeline_desc, None).expect("Could not create camera ray pipeline")
        };

        CameraRays {
            shader,
            set_layout,
            pool,
            layout,
            desc_sets,
            pipeline,
        }
    }

}