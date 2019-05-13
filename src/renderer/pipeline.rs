use gfx_hal::{Backend, Device, pso};
use super::device::DeviceState;

use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Read;

pub struct PipelineState<B: Backend> {
    pub pipeline: Option<B::ComputePipeline>,
    pub pipeline_layout: Option<B::PipelineLayout>,
    pub device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> PipelineState<B> {
    pub unsafe fn new<IS>(
        desc_layouts: IS,
        device_ptr: Rc<RefCell<DeviceState<B>>>,
    ) -> Self
        where
            IS: IntoIterator,
            IS::Item: std::borrow::Borrow<B::DescriptorSetLayout>,
    {
        let device = &device_ptr.borrow().device;

        let pipeline_layout = device
            .create_pipeline_layout(desc_layouts, &[(pso::ShaderStageFlags::COMPUTE, 0..8)])
            .expect("Can't create pipeline layout");

        let comp_module = {
            let glsl = fs::read_to_string("shaders/raytracer.comp").unwrap();
            let spirv: Vec<u8> = glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Compute)
                .unwrap()
                .bytes()
                .map(|b| b.unwrap())
                .collect();
            device.create_shader_module(&spirv).unwrap()
        };

        let pipeline = {
            let comp_entry = pso::EntryPoint::<B> {
                entry: crate::renderer::ENTRY_NAME,
                module: &comp_module,
                specialization: pso::Specialization::default(),
            };

            let mut pipeline_desc = pso::ComputePipelineDesc::new(
                comp_entry,
                &pipeline_layout,
            );

            device.create_compute_pipeline(&pipeline_desc, None).expect("Could not create pipeline")
        };


        PipelineState {
            pipeline: Some(pipeline),
            pipeline_layout: Some(pipeline_layout),
            device: Rc::clone(&device_ptr),
        }
    }
}

impl<B: Backend> Drop for PipelineState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_compute_pipeline(self.pipeline.take().unwrap());
            device.destroy_pipeline_layout(self.pipeline_layout.take().unwrap());
        }
    }
}