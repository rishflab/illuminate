use gfx_hal::{Backend, Device, pso, pool, image as i, format as f, DescriptorPool, command, CommandPool, Compute, command::MultiShot, command::CommandBuffer};
use super::device::DeviceState;
use super::swapchain::SwapchainState;
use crate::renderer::COLOR_RANGE;

use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

pub struct CommandState<B: Backend> {
    pub command_pools: Vec<CommandPool<B, Compute>>,
    pub command_buffers: Vec<CommandBuffer<B, Compute, MultiShot>>,
    pub image_acquire_semaphores: Vec<B::Semaphore>,
    pub free_acquire_semaphore: B::Semaphore,
    pub submission_complete_semaphores: Vec<B::Semaphore>,
    pub submission_complete_fences: Vec<B::Fence>,
    pub frames_in_flight: usize,
    pub frame: usize,
    //device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> CommandState<B> {

    pub unsafe fn new(
        device_state: Rc<RefCell<DeviceState<B>>>,
        frame_images_len: usize,
    ) -> Self {

        let device = &device_state.borrow().device;


        // Define maximum number of frames we want to be able to be "in flight" (being computed
        // simultaneously) at once
        let frames_in_flight = 2;

        // Number of image acquisition semaphores is based on the number of swapchain images, not frames in flight,
        // plus one extra which we can guarantee is unused at any given time by swapping it out with the ones
        // in the rest of the queue.
        let mut image_acquire_semaphores = Vec::with_capacity(frame_images_len);
        let mut free_acquire_semaphore = device.create_semaphore().expect("Could not create semaphore");

        // The number of the rest of the resources is based on the frames in flight.
        let mut submission_complete_semaphores = Vec::with_capacity(frames_in_flight);
        let mut submission_complete_fences = Vec::with_capacity(frames_in_flight);
        // Note: We don't really need a different command pool per frame in such a simple demo like this,
        // but in a more 'real' application, it's generally seen as optimal to have one command pool per
        // thread per frame. There is a flag that lets a command pool reset individual command buffers
        // which are created from it, but by default the whole pool (and therefore all buffers in it)
        // must be reset at once. Furthermore, it is often the case that resetting a whole pool is actually
        // faster and more efficient for the hardware than resetting individual command buffers, so it's
        // usually best to just make a command pool for each set of buffers which need to be reset at the
        // same time (each frame). In our case, each pool will only have one command buffer created from it,
        // though.
        let mut cmd_pools = Vec::with_capacity(frames_in_flight);
        let mut cmd_buffers = Vec::with_capacity(frames_in_flight);

        for _ in 0..frames_in_flight {
            unsafe {
                cmd_pools.push(
                    device
                        .create_command_pool_typed(&device_state.borrow().queues, pool::CommandPoolCreateFlags::empty())
                        .expect("Can't create command pool"),
                );
            }
        }

        for _ in 0..frame_images_len {
            image_acquire_semaphores.push(
                device
                    .create_semaphore()
                    .expect("Could not create semaphore"),
            );
        }

        for i in 0..frames_in_flight {
            submission_complete_semaphores.push(
                device
                    .create_semaphore()
                    .expect("Could not create semaphore"),
            );
            submission_complete_fences.push(
                device
                    .create_fence(true)
                    .expect("Could not create semaphore"),
            );
            cmd_buffers.push(cmd_pools[i].acquire_command_buffer::<command::MultiShot>());
        }

        CommandState {
            command_pools: cmd_pools,
            command_buffers: cmd_buffers,
            image_acquire_semaphores,
            free_acquire_semaphore,
            submission_complete_semaphores,
            submission_complete_fences,
            frames_in_flight,
            frame: 0,
            //device: device_state,
        }
    }


}
//
//impl<B: Backend> Drop for CommandState<B> {
//    fn drop(&mut self) {
//        let device = &self.device.borrow().device;
//
//        unsafe {
//            for command_pool in self.command_pools {
//                device.destroy_command_pool(command_pool);
//            }
//
//            for semaphore in self.image_acquire_semaphores {
//                device.destroy_semaphore(semaphore);
//            }
//
//            device.destroy_semaphore(*self.free_acquire_semaphore);
//
//            for semaphore in self.submission_complete_semaphores {
//                device.destroy_semaphore(semaphore);
//            }
//
//            for fence in self.submission_complete_fences{
//                device.destroy_fence(fence);
//            }
//        }
//    }
//}