extern crate blackhole;

use blackhole::renderer::pathtracer::Pathtracer;
use blackhole::window::WindowState;
use blackhole::renderer::core::backend::{create_backend};
use blackhole::input::{KeyboardState};
use blackhole::scene::{Scene};
use specs::prelude::*;
use blackhole::asset::{load_gltf, MeshData};
use blackhole::scene::mesh::{StaticMeshData, MeshInstance};
use blackhole::scene;
use blackhole::components::*;
use blackhole::resources::*;
use blackhole::window::DIMS;
use blackhole::systems::player_movement::TopDownMovement;
use blackhole::systems::scene_builder::SceneBuilder;
use nalgebra_glm::{vec3, vec3_to_vec4, Quat, quat, quat_angle_axis, quat_look_at, quat_yaw, quat_identity};
use nalgebra_glm as glm;
use blackhole::resources::DeltaTime;
use std::time::Instant;
use std::{thread, time};
use winit::platform::desktop::EventLoopExtDesktop;
use winit::event::{DeviceEvent, VirtualKeyCode, ElementState};

fn main() {
    env_logger::init();

    let mut event_loop = winit::event_loop::EventLoop::new();
    let window_builder = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::LogicalSize::new(1.0, 1.0))
        .with_inner_size(winit::dpi::LogicalSize::new(
            DIMS.width as _,
            DIMS.height as _,
        ))
        .with_title("colour-uniform".to_string());

    let backend = create_backend(window_builder, &event_loop);

    let asset_folder = "assets";
    let gltf = load_gltf(asset_folder, "untitled.gltf")
        .expect("failed to load gltf");

    let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

    let cube_mesh = StaticMeshData {
        id: 0,
        indices: mesh_data.indices.clone(),
        vertices: mesh_data.vertices.clone(),
    };

    let mut world = World::new();

    let mut init = DispatcherBuilder::new()
        .with(SceneBuilder, "scene_builder", &[])
        .build();

    init.setup(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(TopDownMovement, "player_movement", &[])
        .with(SceneBuilder, "scene_builder", &[])
        .build();

    dispatcher.setup(&mut world);

    world.insert(Scene::default());
    world.insert(KeyboardState::default());
    world.insert(DeltaTime::default());

    let player = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(0.0, 0.0, 0.0)))
        .with(Rotation(quat_look_at(&vec3(0.0, 0.0, 1.0), &vec3(0.0, 1.0, 0.0))))
        .with(Scale(glm::vec3(1.0, 3.0, 0.2)))
        .with(Player)
        .build();

    let camera = world.create_entity()
        .with(Position(vec3(0.0, 4.0, -20.0)))
        .with(Rotation(quat_look_at(&vec3(0.0, 0.0, 1.0), &vec3(0.0, 1.0, 0.0))))
        .with(Camera)
        .build();

    let light = world.create_entity()
        .with(PointLight(560.0))
        .with(Position(vec3(1.5, 3.0, -15.0)))
        .build();

    init.dispatch(&world);

    let mut renderer = unsafe {
        Pathtracer::new(backend, &world.fetch::<Scene>())
    };

    let mut start = Instant::now();


    event_loop.run( move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        #[allow(unused_variables)]
            match event {
            winit::event::Event::WindowEvent { event, .. } => {
                match event {
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    }
                    | winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        println!("RedrawRequested");
                        start = Instant::now();
                        //
                        //
                         dispatcher.dispatch(&world);
                        renderer.render(&world.fetch::<Scene>());
                        {
                            let duration = start.elapsed();
                            let mut delta_time = world.write_resource::<DeltaTime>();
                            delta_time.0 = duration;
                            println!("frame time:{:?}", duration);
                        }
                        println!("");
                    }
                    _ => (),
                }
            }
            winit::event::Event::DeviceEvent { event,  .. } => {
                match event {
                    DeviceEvent::Key(keyboard_input) => {
                        let mut keyboard_state = world.write_resource::<KeyboardState>();
                        keyboard_state.process_device_input(keyboard_input);
                    }
                    _ => (),
                }
            }
            winit::event::Event::EventsCleared => {
                println!("EventsCleared");
                renderer.backend.window.request_redraw();
            }
            _ => (),
        }
    });

}