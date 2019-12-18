extern crate gfx_backend_vulkan as back;

use gfx_hal::{
    Limits, Instance, Backend,
    adapter::{MemoryType, Adapter, PhysicalDevice},
};
use crate::window::WindowState;
use crate::input::InputState;
use winit::window::Window;
use std::borrow::Borrow;


pub struct BackendState<B: Backend> {
    pub surface: B::Surface,
    pub adapter: AdapterState<B>,
    //pub window: Window,
}

pub fn create_backend(window_state: &mut WindowState, input_state: &mut InputState) -> (BackendState<back::Backend>, back::Instance) {
    let window = window_state.wb.take().unwrap().build(&input_state.event_loop).unwrap();
    let instance = back::Instance::create("blackhole", 1)
        .expect("Failed to create an instance!");
    let surface = unsafe {
        instance.create_surface(&window).expect("Failed to create a surface!")
    };
    let mut adapters = instance.enumerate_adapters();
    (
        BackendState {
            adapter: AdapterState::new(&mut adapters),
            surface,
            //window,
        },
        instance
    )
}

pub struct AdapterState<B: Backend> {
    pub adapter: Option<Adapter<B>>,
    pub memory_types: Vec<MemoryType>,
    pub limits: Limits,
}

impl<B: Backend> AdapterState<B> {
    pub fn new(adapters: &mut Vec<Adapter<B>>) -> Self {
        print!("Chosen: ");

        for adapter in adapters.iter() {
            println!("{:?}", adapter.info);
        }

        AdapterState::<B>::new_adapter(adapters.remove(0))
    }

    fn new_adapter(adapter: Adapter<B>) -> Self {
        let memory_types = adapter.physical_device.memory_properties().memory_types;
        let limits = adapter.physical_device.limits();
        println!("{:?}", limits);

        AdapterState {
            adapter: Some(adapter),
            memory_types,
            limits,
        }
    }
}
//
//struct AdapterState<B: Backend> {
//    adapter: Option<Adapter<B>>,
//    memory_types: Vec<MemoryType>,
//    limits: hal::Limits,
//}
//
//impl<B: Backend> AdapterState<B> {
//    fn new(adapters: &mut Vec<Adapter<B>>) -> Self {
//        print!("Chosen: ");
//
//        for adapter in adapters.iter() {
//            println!("{:?}", adapter.info);
//        }
//
//        AdapterState::<B>::new_adapter(adapters.remove(0))
//    }
//
//    fn new_adapter(adapter: Adapter<B>) -> Self {
//        let memory_types = adapter.physical_device.memory_properties().memory_types;
//        let limits = adapter.physical_device.limits();
//        println!("{:?}", limits);
//
//        AdapterState {
//            adapter: Some(adapter),
//            memory_types,
//            limits,
//        }
//    }
//}
