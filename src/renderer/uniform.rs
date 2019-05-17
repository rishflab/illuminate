use gfx_hal::{pso, Backend, MemoryType, buffer};

use super::buffer::BufferState;
use super::device::DeviceState;
use super::descriptor::{DescSet, DescSetWrite};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Uniform<B: Backend> {
    pub buffer: Option<BufferState<B>>,
    pub desc: Option<DescSet<B>>,
}


impl<B: Backend> Uniform<B> {
    pub unsafe fn new<T>(
        device: Rc<RefCell<DeviceState<B>>>,
        memory_types: &[MemoryType],
        data: &[T],
        mut desc: DescSet<B>,
        //layout: &B::DescriptorSetLayout,
        binding: u32,
    ) -> Self
        where
            T: Copy,
    {
        let buffer = BufferState::new(
            Rc::clone(&device),
            &data,
            buffer::Usage::TRANSFER_SRC | buffer::Usage::TRANSFER_DST | buffer::Usage::STORAGE,
            memory_types,
        );
        let buffer = Some(buffer);

        desc.write_to_state(
            vec![DescSetWrite {
                binding: binding,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Buffer(
                    buffer.as_ref().unwrap().get_buffer(),
                    None..None,
                )),
            }],
            &mut device.borrow_mut().device,
        );

        Uniform {
            buffer,
            desc: Some(desc),
        }
    }

//    fn get_layout(&self) -> &B::DescriptorSetLayout {
//        self.desc.as_ref().unwrap().get_layout()
//    }
}