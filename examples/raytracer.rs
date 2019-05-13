#![cfg_attr(
not(any(
feature = "vulkan",
feature = "dx11",
feature = "dx12",
feature = "metal",
feature = "gl"
)),
allow(dead_code, unused_extern_crates, unused_imports)
)]

extern crate env_logger;
#[cfg(feature = "dx11")]
extern crate gfx_backend_dx11 as back;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;
extern crate gfx_hal as hal;

extern crate glsl_to_spirv;
extern crate image;
extern crate winit;
extern crate nalgebra_glm as glm;

use hal::format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle};
use hal::pass::Subpass;
use hal::pso::{PipelineStage, ShaderStageFlags};
use hal::queue::Submission;
use hal::{buffer, command, format as f, image as i, memory as m, pass, pool, pso, window::Extent2D, PresentMode};
use hal::{Backend, Backbuffer, QueueFamily, DescriptorPool, FrameSync, Primitive, SwapchainConfig,};
use hal::{Compute, Device, Instance, PhysicalDevice, Surface, Swapchain};

use std::fs;
use std::io::{Cursor, Read};
use std::mem::swap;

#[cfg_attr(rustfmt, rustfmt_skip)]
const DIMS: Extent2D = Extent2D { width: 800, height: 800};

const ENTRY_NAME: &str = "main";

const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
    aspects: f::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

#[cfg(any(
feature = "vulkan",
feature = "dx11",
feature = "dx12",
feature = "metal",
feature = "gl"
))]
fn main() {
    env_logger::init();

    let view = glm::look_at(
        &glm::vec3(1.0,2.0,7.0), // Camera is at (4,3,3), in World Space
        &glm::vec3(0.0,0.0,0.0), // and looks at the origin
        &glm::vec3(0.0,1.0,0.0)  // Head is up (set to 0,-1,0 to look upside-down)
    );

    let view = glm::inverse(&view);

    println!("view mat: {:?}", view.data);


    let mut buffer = view.data.to_vec().clone();


    let mut model = glm::translation(&glm::vec3(0.0, 0.0, 0.0));

    let mut model_vec = model.as_slice().to_vec();
    buffer.append(&mut model_vec);


    let stride = std::mem::size_of::<f32>() as u64;

    let mut events_loop = winit::EventsLoop::new();

    let wb = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(
            DIMS.width as _,
            DIMS.height as _,
        )).with_title("rays".to_string());

    // instantiate backend
    let (_window, _instance, mut adapter, mut surface) = {
        let window = wb.build(&events_loop).unwrap();
        let instance = back::Instance::create("rays", 1);
        let surface = instance.create_surface(&window);
        let adapter = instance.enumerate_adapters().into_iter()
            .find(|a| a.queue_families
                .iter()
                .any(|family| family.supports_compute())
            )
            .expect("Failed to find a GPU with compute support!");

        (window, instance, adapter, surface)
    };

    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let memory_properties = adapter.physical_device.memory_properties();
    let limits = adapter.physical_device.limits();

    // Build a new device and associated command queues
    let (device, mut queue_group) = adapter
        .open_with::<_, Compute>(1, |_family| true)
        .unwrap();

    let mut command_pool = unsafe {
        device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty())
    }.expect("Can't create command pool");

    // Load shader
    let glsl = fs::read_to_string("shaders/raytracer.comp").unwrap();
    let spirv: Vec<u8> = glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Compute)
        .unwrap()
        .bytes()
        .map(|b| b.unwrap())
        .collect();
    let shader = unsafe { device.create_shader_module(&spirv) }.unwrap();

    // Setup pipeline
    let (pipeline_layout, pipeline, set_layout, mut desc_pool) = {
        let set_layout = unsafe {
            device.create_descriptor_set_layout(
                &[
                    pso::DescriptorSetLayoutBinding {
                        binding: 0,
                        ty: pso::DescriptorType::StorageImage,
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::COMPUTE,
                        immutable_samplers: false,
                    },
                    pso::DescriptorSetLayoutBinding {
                        binding: 1,
                        ty: pso::DescriptorType::StorageBuffer,
                        count: 1,
                        stage_flags: pso::ShaderStageFlags::COMPUTE,
                        immutable_samplers: false,
                    },
                ],
                &[],
            )
        }.expect("Can't create descriptor set layout");

        let pipeline_layout = unsafe {
            device.create_pipeline_layout(Some(&set_layout), &[])
        }.expect("Can't create pipeline layout");
        let entry_point = pso::EntryPoint {
            entry: "main",
            module: &shader,
            specialization: pso::Specialization::default(),
        };
        let pipeline = unsafe {
            device.create_compute_pipeline(&pso::ComputePipelineDesc::new(entry_point, &pipeline_layout), None)
        }.expect("Error creating compute pipeline!");

        let desc_pool = unsafe {
            device.create_descriptor_pool(
                8,
                &[
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::StorageImage,
                        count: 4,
                    },
                    pso::DescriptorRangeDesc{
                        ty: pso::DescriptorType::StorageBuffer,
                        count: 4,
                    },
                ],
            )
        }.expect("Can't create descriptor pool");

        println!("{:?}", desc_pool);
        (pipeline_layout, pipeline, set_layout, desc_pool)
    };

    let mut frame_semaphore = device.create_semaphore().expect("Can't create semaphore");
    let mut frame_fence = device.create_fence(false).expect("Can't create fence"); // TODO: remove

    let (caps, formats, _present_modes, _composite_alphas) =
        surface.compatibility(&mut adapter.physical_device);
    println!("formats: {:?}", formats);
    println!("formats: {:?}", caps);

    let format = formats.map_or(f::Format::Rgba8Srgb, |formats| {
        formats
            .iter()
            .find(|format| format.base_format().1 == ChannelType::Srgb)
            .map(|format| *format)
            .unwrap_or(formats[0])
    });

    let mut swap_config = SwapchainConfig::from_caps(&caps, format, DIMS);

    //swap_config.image_count = 3;
    //swap_config.image_layers = 3;
    swap_config.present_mode = hal::PresentMode::Immediate;
    swap_config.image_usage = i::Usage::STORAGE | i::Usage::COLOR_ATTACHMENT ;
    println!("{:?}", swap_config);
    let extent = swap_config.extent.to_extent();

    let (mut swap_chain, mut backbuffer) = unsafe {
        device.create_swapchain(&mut surface, swap_config, None)
    }.expect("Can't create swapchain");

    let (device_memory, device_buffer, device_buffer_size) = unsafe {
        create_buffer::<back::Backend>(
            &device,
            &memory_properties.memory_types,
            m::Properties::DEVICE_LOCAL | m::Properties::CPU_VISIBLE | m::Properties::COHERENT,
            buffer::Usage::TRANSFER_SRC | buffer::Usage::TRANSFER_DST | buffer::Usage::STORAGE,
            stride,
            (buffer.as_slice().len()) as u64,
        )
    };



    // Create The ImageViews
    let mut frame_images = match backbuffer {
        Backbuffer::Images(images) => {
            images
                .into_iter()
                .map(|image| unsafe {

                    //println!("img: {:?}", image);
                    let rtv = device
                        .create_image_view(
                            &image,
                            i::ViewKind::D2,
                            format,
                            Swizzle::NO,
                            COLOR_RANGE.clone(),
                        ).unwrap();

                    println!("img_view: {:?}", rtv);

                    let desc_set = desc_pool.allocate_set(&set_layout).unwrap();

                    device.write_descriptor_sets(Some(
                        pso::DescriptorSetWrite {
                            set: &desc_set,
                            binding: 0,
                            array_offset: 0,
                            descriptors: Some(pso::Descriptor::Image(&rtv, i::Layout::Present)),
                        }
                    ));

                    device.write_descriptor_sets(Some(
                        pso::DescriptorSetWrite {
                            set: &desc_set,
                            binding: 1,
                            array_offset: 0,
                            descriptors: Some(pso::Descriptor::Buffer(&device_buffer, None .. None)),
                        }
                    ));

                    (image, rtv, desc_set)
                }).collect::<Vec<_>>()
        }
        Backbuffer::Framebuffer(_) => unimplemented!("couldnt create image views"),
    };

    let mut running = true;

    let mut x = 0.0;

    while running {



        x += 0.001;

        let mut model = glm::translation(&glm::vec3(x, 0.0, 0.0));

        let mut model_vec = model.as_slice().to_vec();
        let mut buffer = view.data.to_vec().clone();
        buffer.append(&mut model_vec);

        unsafe {
            let mut writer = device.acquire_mapping_writer::<f32>(&device_memory, 0..device_buffer_size).unwrap();
            writer[0..buffer.as_slice().len()].copy_from_slice(buffer.as_slice());
            device.release_mapping_writer(writer).expect("Can't relase mapping writer");
        }


        events_loop.poll_events(|event| {
            if let winit::Event::WindowEvent { event, .. } = event {
                #[allow(unused_variables)]
                    match event {
                    winit::WindowEvent::KeyboardInput {
                        input:
                        winit::KeyboardInput {
                            virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    }
                    | winit::WindowEvent::CloseRequested => running = false,
                    _ => (),
                }
            }
        });

        let frame: hal::SwapImageIndex = unsafe {
            device.reset_fence(&frame_fence).unwrap();
            command_pool.reset();
            match swap_chain.acquire_image(!0, FrameSync::Semaphore(&mut frame_semaphore)) {
                Ok(i) => i,
                Err(_) => {
                    panic!();
                }
            }
        };

        let mut cmd_buffer = command_pool.acquire_command_buffer::<command::OneShot>();
        unsafe {

            cmd_buffer.begin();

            cmd_buffer.bind_compute_pipeline(&pipeline);
            cmd_buffer.bind_compute_descriptor_sets(&pipeline_layout, 0, Some(&frame_images[frame as usize].2), &[]);
            cmd_buffer.dispatch([800, 800, 1]);
            cmd_buffer.finish();

            let submission = Submission {
                command_buffers: Some(&cmd_buffer),
                wait_semaphores: Some((&frame_semaphore, PipelineStage::BOTTOM_OF_PIPE)),
                signal_semaphores: &[],
            };
            queue_group.queues[0].submit(submission, Some(&mut frame_fence));

            // TODO: replace with semaphore
            device.wait_for_fence(&frame_fence, !0).unwrap();
            command_pool.free(Some(cmd_buffer));

            // present frame
            swap_chain.present_nosemaphores(&mut queue_group.queues[0], frame);
        }
    }

    // cleanup!
    device.wait_idle().unwrap();
    unsafe {
        device.destroy_command_pool(command_pool.into_raw());
        device.destroy_descriptor_pool(desc_pool);
        device.destroy_descriptor_set_layout(set_layout);
        device.destroy_fence(frame_fence);
        device.destroy_semaphore(frame_semaphore);
        device.destroy_compute_pipeline(pipeline);
        device.destroy_pipeline_layout(pipeline_layout);
//        for framebuffer in framebuffers {
//            device.destroy_framebuffer(framebuffer);
//        }
//        for (_, rtv, _) in frame_images {
//            device.destroy_image_view(rtv);
//        }

        device.destroy_swapchain(swap_chain);
    }
}



unsafe fn create_buffer<B: Backend>(
    device: &B::Device,
    memory_types: &[hal::MemoryType],
    properties: m::Properties,
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
            requirements.type_mask & (1 << id) != 0 &&
                memory_type.properties.contains(properties)
        })
        .unwrap()
        .into();

    let memory = device.allocate_memory(ty, requirements.size).unwrap();
    device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();

    (memory, buffer, requirements.size)
}

#[cfg(not(any(
feature = "vulkan",
feature = "dx11",
feature = "dx12",
feature = "metal",
feature = "gl"
)))]
fn main() {
    println!("You need to enable the native API feature (vulkan/metal) in order to test the LL");
}