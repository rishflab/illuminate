use gfx_hal::{Adapter, Backend, Compute, QueueGroup, Surface, queue};

pub struct DeviceState<B: Backend> {
    pub device: B::Device,
    pub physical_device: B::PhysicalDevice,
    pub queues: QueueGroup<B, Compute>,
}

impl<B: Backend> DeviceState<B> {
    pub fn new(adapter: Adapter<B>, surface: &B::Surface) -> Self {

        println!("creating queue");

        let (device, queue_group) = adapter
            .open_with::<_, Compute>(1, |family| surface.supports_queue_family(family))
            .unwrap();


        println!("queue created");

        DeviceState {
            device: device,
            queues: queue_group,
            physical_device: adapter.physical_device,
        }
    }

    pub fn get_queue_family_id(&self) -> queue::QueueFamilyId {
        self.queues.family()
    }
}