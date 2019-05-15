use gfx_hal::{Backend, Device, Surface, SwapchainConfig, pso, pool, image as i, format as f, DescriptorPool};
use super::device::DeviceState;
use super::swapchain::SwapchainState;
use super::descriptor::{DescSetWrite, DescSet, DescSetLayout};
use crate::renderer::COLOR_RANGE;

use std::cell::RefCell;
use std::rc::Rc;

pub struct FramebufferState<B: Backend> {
    command_pools: Option<Vec<gfx_hal::CommandPool<B, gfx_hal::Compute>>>,
    framebuffer_fences: Option<Vec<B::Fence>>,
    frame_images: Option<Vec<(B::Image, B::ImageView)>>,
    acquire_semaphores: Option<Vec<B::Semaphore>>,
    present_semaphores: Option<Vec<B::Semaphore>>,
    last_ref: usize,
    device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> FramebufferState<B> {
    pub unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        swapchain: &mut SwapchainState<B>,
    ) -> Self {

        let frame_images = {
            let extent = i::Extent {
                width: swapchain.extent.width as _,
                height: swapchain.extent.height as _,
                depth: 1,
            };


            let pairs = swapchain.backbuffer.take().unwrap()
                .into_iter()
                .map(|image| {
                    println!("creating image view");
                    let rtv = device
                        .borrow()
                        .device
                        .create_image_view(
                            &image,
                            i::ViewKind::D2,
                            swapchain.format,
                            f::Swizzle::NO,
                            COLOR_RANGE.clone(),
                        )
                        .unwrap();

                    println!("{:?}", rtv);


                    (image, rtv)
                })
                .collect::<Vec<_>>();
            pairs
        };

        let iter_count = if frame_images.len() != 0 {
            frame_images.len()
        } else {
            1 // GL can have zero
        };

        let mut fences: Vec<B::Fence> = vec![];
        let mut command_pools: Vec<gfx_hal::CommandPool<B, gfx_hal::Compute>> = vec![];
        let mut acquire_semaphores: Vec<B::Semaphore> = vec![];
        let mut present_semaphores: Vec<B::Semaphore> = vec![];

        for _ in 0..iter_count {
            fences.push(device.borrow().device.create_fence(true).unwrap());
            command_pools.push(
                device
                    .borrow()
                    .device
                    .create_command_pool_typed(
                        &device.borrow().queues,
                        pool::CommandPoolCreateFlags::empty(),
                    )
                    .expect("Can't create command pool"),
            );

            acquire_semaphores.push(device.borrow().device.create_semaphore().unwrap());
            present_semaphores.push(device.borrow().device.create_semaphore().unwrap());
        }

        FramebufferState {
            framebuffer_fences: Some(fences),
            frame_images: Some(frame_images),
            command_pools: Some(command_pools),
            present_semaphores: Some(present_semaphores),
            acquire_semaphores: Some(acquire_semaphores),
            device,
            last_ref: 0,
        }
    }

    pub unsafe fn write_descriptor_sets(
        &mut self,
        device: Rc<RefCell<DeviceState<B>>>,
        desc_layout: &B::DescriptorSetLayout,
        desc_pool: &mut B::DescriptorPool,
    ) -> Vec<B::DescriptorSet> {


        self.frame_images
            .as_ref()
            .unwrap()
            .iter()
            .map(|(_, view)| {
                let desc_set = desc_pool.allocate_set(desc_layout).unwrap();

                device
                    .borrow()
                    .device
                    .write_descriptor_sets(Some(
                        pso::DescriptorSetWrite {
                            set: &desc_set,
                            binding: 0,
                            array_offset: 0,
                            descriptors: Some(pso::Descriptor::Image(view, i::Layout::Present)),
                        }
                    ));


                desc_set
            })
            .collect::<Vec<_>>()

    }

    pub fn next_acq_pre_pair_index(&mut self) -> usize {
        if self.last_ref >= self.acquire_semaphores.as_ref().unwrap().len() {
            self.last_ref = 0
        }

        let ret = self.last_ref;
        self.last_ref += 1;
        ret
    }

    pub fn get_frame_data(
        &mut self,
        frame_id: Option<usize>,
        sem_index: Option<usize>,
    ) -> (
        Option<(
            &mut B::Fence,
            &mut gfx_hal::CommandPool<B, ::gfx_hal::Compute>,
        )>,
        Option<(&mut B::Semaphore, &mut B::Semaphore)>,
    ) {
        (
            if let Some(fid) = frame_id {
                Some((
                    &mut self.framebuffer_fences.as_mut().unwrap()[fid],
                    &mut self.command_pools.as_mut().unwrap()[fid],
                ))
            } else {
                None
            },
            if let Some(sid) = sem_index {
                Some((
                    &mut self.acquire_semaphores.as_mut().unwrap()[sid],
                    &mut self.present_semaphores.as_mut().unwrap()[sid],
                ))
            } else {
                None
            },
        )
    }
}

impl<B: Backend> Drop for FramebufferState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;

        unsafe {

            for command_pool in self.command_pools.take().unwrap() {
                device.destroy_command_pool(command_pool.into_raw());
            }

            for acquire_semaphore in self.acquire_semaphores.take().unwrap() {
                device.destroy_semaphore(acquire_semaphore);
            }

            for present_semaphore in self.present_semaphores.take().unwrap() {
                device.destroy_semaphore(present_semaphore);
            }

            for (_, rtv) in self.frame_images.take().unwrap() {
                device.destroy_image_view(rtv);
            }
        }
    }
}