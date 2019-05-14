extern crate gfx_backend_vulkan as back;

use gfx_hal::{Adapter, Limits, Instance, MemoryType, Backend, PhysicalDevice};
use super::window::WindowState;


pub struct BackendState<B: Backend> {
    pub surface: B::Surface,
    pub adapter: AdapterState<B>,

    pub window: winit::Window,
}

pub fn create_backend(window_state: &mut WindowState) -> (BackendState<back::Backend>, back::Instance) {
    let window = window_state
        .wb
        .take()
        .unwrap()
        .build(&window_state.events_loop)
        .unwrap();
    let instance = back::Instance::create("gfx-rs quad", 1);
    let surface = instance.create_surface(&window);
    let mut adapters = instance.enumerate_adapters();
    (
        BackendState {
            adapter: AdapterState::new(&mut adapters),
            surface,
            window,
        },
        instance,
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
