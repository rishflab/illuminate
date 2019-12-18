use gfx_hal::{Backend, queue, prelude::*, queue::QueueGroup, adapter::Adapter};

pub struct DeviceState<B: Backend> {
    pub device: B::Device,
    pub physical_device: B::PhysicalDevice,
    pub queues: QueueGroup<B>,
}

impl<B: Backend> DeviceState<B> {
    pub unsafe fn new(adapter: Adapter<B>, surface: &B::Surface) -> Self {

        println!("creating queue");

        let family = adapter
            .queue_families
            .iter()
            .find(|family| {
                surface.supports_queue_family(family) && family.queue_type().supports_compute()
            })
            .unwrap();

        let mut gpu = adapter.physical_device.open(&[(family, &[1.0])], gfx_hal::Features::empty())
            .unwrap();

        println!("queue created");

        DeviceState {
            device: gpu.device,
            queues: gpu.queue_groups.pop().unwrap(),
            physical_device: adapter.physical_device,
        }
    }

    pub fn get_command_queue(&mut self) -> &mut B::CommandQueue {
        &mut self.queues.queues[0]
    }

    pub fn get_queue_family_id(&self) -> queue::QueueFamilyId {
        self.queues.family
    }
}