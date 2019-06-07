use gfx_hal::{Device, Backend, DescriptorPool, pso};

use super::device::DeviceState;

use std::cell::RefCell;
use std::rc::Rc;

pub struct DescSetLayout<B: Backend> {
    pub layout: Option<B::DescriptorSetLayout>,
    pub device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> DescSetLayout<B> {
    pub unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        bindings: Vec<pso::DescriptorSetLayoutBinding>,
    ) -> DescSetLayout<B> {

        let desc_set_layout = device
            .borrow()
            .device
            .create_descriptor_set_layout(bindings, &[])
            .ok();

        println!("created desc_set_layout");

        DescSetLayout {
            layout: desc_set_layout,
            device,
        }
    }

    pub unsafe fn create_desc_set(&self, desc_pool: &mut B::DescriptorPool, buffer: &B::Buffer) -> B::DescriptorSet {

        let desc_set = desc_pool
            .allocate_set(self.layout.as_ref().unwrap())
            .expect("could not allocate descriptor set");

        self.device
            .borrow()
            .device
            .write_descriptor_sets(Some(
                pso::DescriptorSetWrite {
                    set: &desc_set,
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(buffer, None..None)),
                }
            ));

        desc_set

    }

    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.layout.as_ref().unwrap()
    }
}

impl<B: Backend> Drop for DescSetLayout<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_descriptor_set_layout(self.layout.take().unwrap());
        }
    }
}
