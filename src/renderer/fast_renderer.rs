#![allow(clippy::len_zero)]
#![allow(clippy::many_single_char_names)]

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;


use arrayvec::ArrayVec;
use core::mem::{size_of, ManuallyDrop};
use gfx_hal::{
    adapter::{Adapter, MemoryTypeId, PhysicalDevice},
    buffer::Usage as BufferUsage,
    buffer,
    command::{ClearColor, ClearValue, CommandBuffer, MultiShot, Primary},
    device::Device,
    format::{Aspects, ChannelType, Format, Swizzle},
    image::{Kind, Tiling, Size, ViewCapabilities, Extent, Layout, SubresourceRange, Usage, ViewKind},
    memory::{self, Properties, Requirements},
    pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDesc},
    pool::{CommandPool, CommandPoolCreateFlags},
    pso::{
        self, AttributeDesc, BakedStates, BasePipeline, BlendDesc, BlendOp, BlendState, ColorBlendDesc,
        ColorMask, DepthStencilDesc, DepthTest, DescriptorSetLayoutBinding, Element, EntryPoint, Face,
        Factor, FrontFace, GraphicsPipelineDesc, GraphicsShaderSet, InputAssemblerDesc, LogicOp,
        PipelineCreationFlags, PipelineStage, PolygonMode, Rasterizer, Rect, ShaderStageFlags,
        Specialization, StencilTest, VertexBufferDesc, Viewport, DescriptorPool,
    },
    queue::{family::QueueGroup, Submission},
    window::{Backbuffer, Extent2D, FrameSync, PresentMode, Swapchain, SwapchainConfig},
    Backend, Gpu, Graphics, Instance, Primitive, QueueFamily, Surface,
};

use winit::{
    dpi::LogicalSize, CreationError, Event, EventsLoop, Window, WindowBuilder, WindowEvent,
};

use super::mesh::Triangle;
use specs::storage::UnprotectedStorage;


pub const VERTEX_SOURCE: &str = "#version 450
layout (location = 0) in vec2 position;

out gl_PerVertex {
  vec4 gl_Position;
};

void main()
{
  gl_Position = vec4(position, 0.0, 1.0);
}";

pub const FRAGMENT_SOURCE: &str = "#version 450
layout(location = 0) out vec4 color;

void main()
{
  color = vec4(1.0);
}";


pub struct HalState {
    buffer: ManuallyDrop<<back::Backend as Backend>::Buffer>,
    memory: ManuallyDrop<<back::Backend as Backend>::Memory>,
    descriptor_set_layouts: Vec<<back::Backend as Backend>::DescriptorSetLayout>,
    pipeline_layout: ManuallyDrop<<back::Backend as Backend>::PipelineLayout>,
    graphics_pipeline: ManuallyDrop<<back::Backend as Backend>::GraphicsPipeline>,
    requirements: Requirements,
    current_frame: usize,
    frames_in_flight: usize,
    in_flight_fences: Vec<<back::Backend as Backend>::Fence>,
    render_finished_semaphores: Vec<<back::Backend as Backend>::Semaphore>,
    image_available_semaphores: Vec<<back::Backend as Backend>::Semaphore>,
    command_buffers: Vec<CommandBuffer<back::Backend, Graphics, MultiShot, Primary>>,
    command_pool: ManuallyDrop<CommandPool<back::Backend, Graphics>>,
    framebuffers: Vec<<back::Backend as Backend>::Framebuffer>,
    image_views: Vec<(<back::Backend as Backend>::ImageView)>,
    render_pass: ManuallyDrop<<back::Backend as Backend>::RenderPass>,
    render_area: Rect,
    queue_group: QueueGroup<back::Backend, Graphics>,
    swapchain: ManuallyDrop<<back::Backend as Backend>::Swapchain>,
    device: ManuallyDrop<back::Device>,
    _adapter: Adapter<back::Backend>,
    _surface: <back::Backend as Backend>::Surface,
    _instance: ManuallyDrop<back::Instance>,
}

impl HalState {
    /// Creates a new, fully initialized HalState.
    pub fn new(window: &Window)
               -> Result<(), &'static str>
//        -> Result<Self, &'static str>
    {
        // Create An Instance
        let instance = back::Instance::create(super::WINDOW_NAME, 1);

        //info!("{:?}", instance);

        // Create A Surface
        let mut surface = instance.create_surface(window);

        let mut adapters = instance.enumerate_adapters();

//        // Select An Adapter
//        let adapter = instance
//            .enumerate_adapters()
//            .into_iter()
//            .find(|a| {
//
//                a.queue_families
//                    .iter()
//                    .any(|qf| qf.supports_graphics() && surface.supports_queue_family(qf))
//            })
//            .ok_or("Couldn't find a graphical Adapter!")?;

        let adapter = adapters.remove(0);
        let memory_types = adapter.physical_device.memory_properties().memory_types;
        let memory_properties = adapter.physical_device.memory_properties();
        let limits = adapter.physical_device.limits();

        info!("selected adapter: {:?}", adapter.info);

        // Open A Device and take out a QueueGroup
        let (mut device, queue_group) = {
            let queue_family = adapter
                .queue_families
                .iter()
                .find(|qf| qf.supports_graphics() && surface.supports_queue_family(qf))
                .ok_or("Couldn't find a QueueFamily with graphics!")?;
            let Gpu { device, mut queues } = unsafe {
                adapter
                    .physical_device
                    .open(&[(&queue_family, &[1.0; 1])])
                    .map_err(|_| "Couldn't open the PhysicalDevice!")?
            };
            let queue_group = queues
                .take::<Graphics>(queue_family.id())
                .ok_or("Couldn't take ownership of the QueueGroup!")?;
            if queue_group.queues.len() > 0 {
                Ok(())
            } else {
                Err("The QueueGroup did not have any CommandQueues available!")
            }?;
            (device, queue_group)
        };

        // Create A Swapchain, this is extra long
        let (mut swapchain, extent, backbuffer, format, frames_in_flight) = {
            let (caps, preferred_formats, present_modes, composite_alphas) =
                surface.compatibility(&adapter.physical_device);
            info!("{:?}", caps);
            info!("Preferred Formats: {:?}", preferred_formats);
            info!("Present Modes: {:?}", present_modes);
            info!("Composite Alphas: {:?}", composite_alphas);
            let present_mode = {
                use gfx_hal::window::PresentMode::*;
                [Mailbox, Fifo, Relaxed, Immediate]
                    .iter()
                    .cloned()
                    .find(|pm| present_modes.contains(pm))
                    .ok_or("No PresentMode values specified!")?
            };
            let composite_alpha = {
                use gfx_hal::window::CompositeAlpha::*;
                [Opaque, Inherit, PreMultiplied, PostMultiplied]
                    .iter()
                    .cloned()
                    .find(|ca| composite_alphas.contains(ca))
                    .ok_or("No CompositeAlpha values specified!")?
            };
            let format = match preferred_formats {
                None => Format::Bgra8Srgb,
                Some(formats) => match formats
                    .iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .cloned()
                    {
                        Some(srgb_format) => srgb_format,
                        None => formats
                            .get(0)
                            .cloned()
                            .ok_or("Preferred format list was empty!")?,
                    },
            };
            let extent = {
                let window_client_area = window
                    .get_inner_size()
                    .ok_or("Window doesn't exist!")?
                    .to_physical(window.get_hidpi_factor());
                Extent2D {
                    width: caps.extents.end.width.min(window_client_area.width as u32),
                    height: caps
                        .extents
                        .end
                        .height
                        .min(window_client_area.height as u32),
                }
            };
            let image_count = if present_mode == PresentMode::Mailbox {
                (caps.image_count.end - 1).min(3)
            } else {
                (caps.image_count.end - 1).min(2)
            };
            let image_layers = 1;

//            let image_usage = if caps.usage.contains(Usage::COLOR_ATTACHMENT | Usage::STORAGE ) {
//                Usage::COLOR_ATTACHMENT | Usage::STORAGE
//            } else {
//                error!("The Surface isn't capable of supporting color attachment and storage!");
//                Err("The Surface isn't capable of supporting color attachment and storage!")?
//            };

            let image_usage = if caps.usage.contains(Usage::COLOR_ATTACHMENT | Usage::TRANSFER_DST) {
                Usage::COLOR_ATTACHMENT | Usage::TRANSFER_DST
            } else {
                error!("The Surface isn't capable of supporting usage!");
                Err("The Surface isn't capable of supporting usage!")?
            };

            let swapchain_config = SwapchainConfig {
                present_mode,
                composite_alpha,
                format,
                extent,
                image_count,
                image_layers,
                image_usage,
            };
            info!("{:?}", swapchain_config);

            let (swapchain, backbuffer) = unsafe {
                device
                    .create_swapchain(&mut surface, swapchain_config, None)
                    .map_err(|_| "Failed to create the swapchain!")?
            };
            (swapchain, extent, backbuffer, format, image_count as usize)
        };

        // Create Our Sync Primitives
        let (image_available_semaphores, render_finished_semaphores, in_flight_fences) = {
            let mut image_available_semaphores: Vec<<back::Backend as Backend>::Semaphore> = vec![];
            let mut render_finished_semaphores: Vec<<back::Backend as Backend>::Semaphore> = vec![];
            let mut in_flight_fences: Vec<<back::Backend as Backend>::Fence> = vec![];
            for _ in 0..frames_in_flight {
                in_flight_fences.push(
                    device
                        .create_fence(true)
                        .map_err(|_| "Could not create a fence!")?,
                );
                image_available_semaphores.push(
                    device
                        .create_semaphore()
                        .map_err(|_| "Could not create a semaphore!")?,
                );
                render_finished_semaphores.push(
                    device
                        .create_semaphore()
                        .map_err(|_| "Could not create a semaphore!")?,
                );
            }
            (
                image_available_semaphores,
                render_finished_semaphores,
                in_flight_fences,
            )
        };


        // Create The ImageViews
        let image_views: Vec<_> = match backbuffer {
            Backbuffer::Images(images) => images
                .into_iter()
                .map(|image| unsafe {
                    device
                        .create_image_view(
                            &image,
                            ViewKind::D2,
                            format,
                            Swizzle::NO,
                            SubresourceRange {
                                aspects: Aspects::COLOR,
                                levels: 0..1,
                                layers: 0..1,
                            },
                        )
                        .map_err(|_| "Couldn't create the image_view for the image!")
                })
                .collect::<Result<Vec<_>, &str>>()?,
            Backbuffer::Framebuffer(_) => unimplemented!("Can't handle framebuffer backbuffer!"),
        };

        info!("{:?}", image_views);

        // Create Our CommandPool
        let mut command_pool = unsafe {
            device
                .create_command_pool_typed(&queue_group, CommandPoolCreateFlags::RESET_INDIVIDUAL)
                .map_err(|_| "Could not create the raw command pool!")?
        };

        // Setup layout
        let set_layout = unsafe {
            device.create_descriptor_set_layout(
                &[
                    pso::DescriptorSetLayoutBinding {
                        binding: 0,
                        ty: pso::DescriptorType::StorageBuffer,
                        count: 1,
                        stage_flags: ShaderStageFlags::COMPUTE,
                        immutable_samplers: false,
                    },
                    pso::DescriptorSetLayoutBinding {
                        binding: 1,
                        ty: pso::DescriptorType::StorageImage,
                        count: 1,
                        stage_flags: ShaderStageFlags::COMPUTE,
                        immutable_samplers: false,
                    },
                ],
                &[],
            )
        }.expect("Can't create descriptor set layout");

        let pipeline_layout = unsafe { device.create_pipeline_layout(Some(&set_layout), &[]) }
            .expect("Can't create pipeline layout");

        // Descriptors
        let mut desc_pool = unsafe {
            device.create_descriptor_pool(
                1, // sets
                &[
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::StorageBuffer,
                        count: 1,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::StorageImage,
                        count: 1,
                    },
                ],
            )
        }
            .expect("Can't create descriptor pool");

        let numbers: Vec<u32> = vec![200, 100, 130, 12];

        let stride = std::mem::size_of::<u32>() as u64;

        let (staging_memory, staging_buffer, staging_size) = unsafe {
            create_buffer::<back::Backend>(
                &device,
                &memory_properties.memory_types,
                memory::Properties::CPU_VISIBLE | memory::Properties::COHERENT,
                buffer::Usage::TRANSFER_SRC | buffer::Usage::TRANSFER_DST,
                stride,
                numbers.len() as u64,
            )
        };

        unsafe {
            let mut writer = device
                .acquire_mapping_writer::<u32>(&staging_memory, 0..staging_size)
                .unwrap();
            writer[0..numbers.len()].copy_from_slice(&numbers);
            device
                .release_mapping_writer(writer)
                .expect("Can't relase mapping writer");
        }

        info!("memory types: {:?}", memory_types);

        let image_available = &image_available_semaphores[0];

        let (i_u32, i_usize) = unsafe {
            let image_index = swapchain
                .acquire_image(core::u64::MAX, FrameSync::Semaphore(image_available))
                .map_err(|_| "Couldn't acquire an image from the swapchain!")?;
            (image_index, image_index as usize)
        };


        let desc_set;

        unsafe {
            desc_set = desc_pool.allocate_set(&set_layout).unwrap();
            device.write_descriptor_sets(Some(pso::DescriptorSetWrite {
                set: &desc_set,
                binding: 0,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Buffer(&staging_buffer, None..None)),
            }));
            device.write_descriptor_sets(Some(pso::DescriptorSetWrite {
                set: &desc_set,
                binding: 1,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Image(image_views.get(i_usize).unwrap(), Layout::Present)),
            }));
        };


//        Ok(Self {
//            requirements,
//            buffer: ManuallyDrop::new(buffer),
//            memory: ManuallyDrop::new(memory),
//            _instance: ManuallyDrop::new(instance),
//            _surface: surface,
//            _adapter: adapter,
//            device: ManuallyDrop::new(device),
//            queue_group,
//            swapchain: ManuallyDrop::new(swapchain),
//            render_area: extent.to_extent().rect(),
//            render_pass: ManuallyDrop::new(render_pass),
//            image_views,
//            framebuffers,
//            command_pool: ManuallyDrop::new(command_pool),
//            command_buffers,
//            image_available_semaphores,
//            render_finished_semaphores,
//            in_flight_fences,
//            frames_in_flight,
//            current_frame: 0,
//            descriptor_set_layouts,
//            pipeline_layout: ManuallyDrop::new(pipeline_layout),
//            graphics_pipeline: ManuallyDrop::new(graphics_pipeline),
//        })

        Ok(())
    }
}

impl core::ops::Drop for HalState {
    /// We have to clean up "leaf" elements before "root" elements. Basically, we
    /// clean up in reverse of the order that we created things.
    fn drop(&mut self) {
        let _ = self.device.wait_idle();
        unsafe {
            for descriptor_set_layout in self.descriptor_set_layouts.drain(..) {
                self
                    .device
                    .destroy_descriptor_set_layout(descriptor_set_layout)
            }
            for fence in self.in_flight_fences.drain(..) {
                self.device.destroy_fence(fence)
            }
            for semaphore in self.render_finished_semaphores.drain(..) {
                self.device.destroy_semaphore(semaphore)
            }
            for semaphore in self.image_available_semaphores.drain(..) {
                self.device.destroy_semaphore(semaphore)
            }
            for framebuffer in self.framebuffers.drain(..) {
                self.device.destroy_framebuffer(framebuffer);
            }
            for image_view in self.image_views.drain(..) {
                self.device.destroy_image_view(image_view);
            }
            // LAST RESORT STYLE CODE, NOT TO BE IMITATED LIGHTLY
            use core::ptr::read;
            self
                .device
                .destroy_buffer(ManuallyDrop::into_inner(read(&self.buffer)));
            self
                .device
                .free_memory(ManuallyDrop::into_inner(read(&self.memory)));
            self
                .device
                .destroy_pipeline_layout(ManuallyDrop::into_inner(read(&self.pipeline_layout)));
            self
                .device
                .destroy_graphics_pipeline(ManuallyDrop::into_inner(read(&self.graphics_pipeline)));
            self
                .device
                .destroy_command_pool(ManuallyDrop::into_inner(read(&self.command_pool)).into_raw());
            self
                .device
                .destroy_render_pass(ManuallyDrop::into_inner(read(&self.render_pass)));
            self
                .device
                .destroy_swapchain(ManuallyDrop::into_inner(read(&self.swapchain)));
            ManuallyDrop::drop(&mut self.device);
            ManuallyDrop::drop(&mut self._instance);
        }
    }
}

unsafe fn create_buffer<B: Backend>(
    device: &B::Device,
    memory_types: &[gfx_hal::MemoryType],
    properties: memory::Properties,
    usage: buffer::Usage,
    stride: u64,
    len: u64,
) -> (B::Memory, B::Buffer, u64) {
    let mut buffer = device.create_buffer(stride * len, usage).unwrap();
    let requirements = device.get_buffer_requirements(&buffer);

    let ty = memory_types
        .into_iter()
        .enumerate()
        .position(|(id, memory_type)| {
            requirements.type_mask & (1 << id) != 0 && memory_type.properties.contains(properties)
        })
        .unwrap()
        .into();

    let memory = device.allocate_memory(ty, requirements.size).unwrap();
    device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();

    (memory, buffer, requirements.size)
}