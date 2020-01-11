use gfx_hal::{
    device::Device,
    Backend, buffer, memory as m,
    adapter::MemoryType
};
use super::device::DeviceState;
use std::cell::RefCell;
use std::rc::Rc;
use std::ptr;
use core::mem::size_of;


pub struct BufferState<B: Backend> {
    pub memory: Option<B::Memory>,
    pub buffer: Option<B::Buffer>,
    pub device: Rc<RefCell<DeviceState<B>>>,
    pub size: u64,
}

impl<B: Backend> BufferState<B> {
    pub fn get_buffer(&self) -> &B::Buffer {
        self.buffer.as_ref().unwrap()
    }

    pub unsafe fn empty<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        memory_types: &[MemoryType],
        memory_properties: m::Properties,
        usage: buffer::Usage,
        length: u64,
        _t: T) -> Self{

        let (memory, buffer, size) = {

            let device = &device_ptr.borrow().device;

            let stride = size_of::<T>() as u64;
            println!("size of: {:?}", stride);
            let upload_size = length as u64 * stride;

            let mut buffer = device.create_buffer(upload_size, usage).unwrap();

            let mem_req = device.get_buffer_requirements(&buffer);

            let upload_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(memory_properties)
                })
                .unwrap()
                .into();

            println!("memory upload type: {:?}", upload_type);

            let memory = device.allocate_memory(upload_type, mem_req.size).unwrap();

            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();

            println!("full size: {:?}", mem_req.size);

            (memory, buffer, mem_req.size)

        };

        BufferState {
            memory: Some(memory),
            buffer: Some(buffer),
            device: device_ptr,
            size: size,
        }
    }

    pub unsafe fn new<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        memory_types: &[MemoryType],
        memory_properties: m::Properties,
        usage: buffer::Usage,
        data_source: &[T],
    ) -> Self
        where
            T: Copy,
    {
        let memory: B::Memory;
        let mut buffer: B::Buffer;
        let size: u64;

        let stride = size_of::<T>();
        let upload_size = data_source.len() * stride;

        {
            let device = &device_ptr.borrow().device;

            buffer = device.create_buffer(upload_size as u64, usage).unwrap();
            let mem_req = device.get_buffer_requirements(&buffer);

            // A note about performance: Using CPU_VISIBLE memory is convenient because it can be
            // directly memory mapped and easily updated by the CPU, but it is very slow and so should
            // only be used for small pieces of data that need to be updated very frequently. For something like
            // a vertex buffer that may be much larger and should not change frequently, you should instead
            // use a DEVICE_LOCAL buffer that gets filled by copying data from a CPU_VISIBLE staging buffer.
            let upload_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(memory_properties)
                })
                .unwrap()
                .into();

            memory = device.allocate_memory(upload_type, mem_req.size).unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
            size = mem_req.size;

            // TODO: check transitions: read/write mapping and vertex buffer read
            let mapping = device.map_memory(&memory, 0 .. size).unwrap();
            ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
            device.unmap_memory(&memory);
        }

        BufferState {
            memory: Some(memory),
            buffer: Some(buffer),
            device: device_ptr,
            size,
        }
    }

//    pub unsafe fn new<T>(
//        device_ptr: Rc<RefCell<DeviceState<B>>>,
//        memory_types: &[MemoryType],
//        memory_properties: m::Properties,
//        usage: buffer::Usage,
//        data: &[T],
//    ) -> Self
//        where
//            T: Copy + std::fmt::Debug,
//    {
//
//        println!("data to write: {:?}", data);
//
//        let (buffer, memory, size) = {
//
//            let memory: B::Memory;
//            let mut buffer: B::Buffer;
//            let size: u64;
//
//            let stride = size_of::<T>() as u64;
//            let upload_size = data.len() as u64 * stride;
//
//            println!("upload size: {:?}", upload_size);
//
//            {
//                let device = &device_ptr.borrow().device;
//
//                buffer = device.create_buffer(upload_size, usage).unwrap();
//                let mem_req = device.get_buffer_requirements(&buffer);
//
//                let upload_type = memory_types
//                    .iter()
//                    .enumerate()
//                    .position(|(id, mem_type)| {
//                        mem_req.type_mask & (1 << id) != 0
//                            && mem_type.properties.contains(memory_properties)
//                    })
//                    .unwrap()
//                    .into();
//
//                println!("memory upload type: {:?}", upload_type);
//
//                memory = device.allocate_memory(upload_type, mem_req.size).unwrap();
//                device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
//                size = mem_req.size;
//
//                // TODO: check transitions: read/write mapping and vertex buffer read
//                {
//                    let mut data_target = device
//                        .acquire_mapping_writer::<T>(&memory, 0..size)
//                        .unwrap();
//                    data_target[0..data.len()].copy_from_slice(data);
//                    device.release_mapping_writer(data_target).unwrap();
//                }
//
//            }
//
//            println!("memory written");
//
//            (buffer, memory, size)
//        };
//
//
//        BufferState {
//            memory: Some(memory),
//            buffer: Some(buffer),
//            device: device_ptr,
//            size,
//        }
//    }

//    unsafe fn init_data<T>(
//        device_ptr: Rc<RefCell<DeviceState<B>>>,
//        data_source: &[T],
//        usage: buffer::Usage,
//        memory_types: &[MemoryType],
//    ) -> (B::Buffer, B::Memory, u64)
//        where
//            T: Copy,
//    {
//        let memory: B::Memory;
//        let mut buffer: B::Buffer;
//        let size: u64;
//
//        let stride = size_of::<T>() as u64;
//        let upload_size = data_source.len() as u64 * stride;
//
//        println!("upload size: {:?}", upload_size);
//
//        {
//            let device = &device_ptr.borrow().device;
//
//            buffer = device.create_buffer(upload_size, usage).unwrap();
//            let mem_req = device.get_buffer_requirements(&buffer);
//
//            // A note about performance: Using CPU_VISIBLE memory is convenient because it can be
//            // directly memory mapped and easily updated by the CPU, but it is very slow and so should
//            // only be used for small pieces of data that need to be updated very frequently. For something like
//            // a vertex buffer that may be much larger and should not change frequently, you should instead
//            // use a DEVICE_LOCAL buffer that gets filled by copying data from a CPU_VISIBLE staging buffer.
//            let upload_type = memory_types
//                .iter()
//                .enumerate()
//                .position(|(id, mem_type)| {
//                    mem_req.type_mask & (1 << id) != 0
//                        && mem_type.properties.contains(m::Properties::DEVICE_LOCAL | m::Properties::CPU_VISIBLE | m::Properties::COHERENT)
//                })
//                .unwrap()
//                .into();
//
//            memory = device.allocate_memory(upload_type, mem_req.size).unwrap();
//            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
//            size = mem_req.size;
//
//            // TODO: check transitions: read/write mapping and vertex buffer read
//            {
//                let mut data_target = device
//                    .acquire_mapping_writer::<T>(&memory, 0..size)
//                    .unwrap();
//                data_target[0..data_source.len()].copy_from_slice(data_source);
//                device.release_mapping_writer(data_target).unwrap();
//            }
//
//        }
//
//        println!("memory written");
//
//        (buffer, memory, size)
//
//    }

//    pub fn update_data<T>(
//        &mut self, offset:
//        u64, data_source: &[T])
//        where
//            T: Copy,
//    {
//        let device = &self.device.borrow().device;
//
//        let stride = size_of::<T>() as u64;
//        let upload_size = data_source.len() as u64 * stride;
//
//        assert!(offset + upload_size <= self.size);
//
//        unsafe {
//            let mut data_target = device
//                .acquire_mapping_writer::<T>(self.memory.as_ref().unwrap(), offset..self.size)
//                .unwrap();
//            data_target[0..data_source.len()].copy_from_slice(data_source);
//            device.release_mapping_writer(data_target).unwrap();
//        }
//    }

    pub fn update_data<T>(&mut self, offset: u64, data_source: &[T])
        where
            T: Copy,
    {
        let device = &self.device.borrow().device;

        let stride = size_of::<T>();
        let upload_size = data_source.len() * stride;

        assert!(offset + upload_size as u64 <= self.size);
        let memory = self.memory.as_ref().unwrap();

        unsafe {
            let mapping = device.map_memory(memory, offset .. self.size).unwrap();
            ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
            device.unmap_memory(memory);
        }
    }
}


impl<B: Backend> Drop for BufferState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_buffer(self.buffer.take().unwrap());
            device.free_memory(self.memory.take().unwrap());
        }
    }
}