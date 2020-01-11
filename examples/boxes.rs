extern crate not_mechanical_engine as engine;

use engine::renderer::pathtracer::Pathtracer;
use engine::window::WindowState;
use engine::renderer::core::backend::{create_backend};
use engine::scene::{Scene};
use specs::prelude::*;
use engine::asset::{load_gltf, MeshData};
use engine::scene::mesh::{StaticMeshData, MeshInstance};
use engine::scene;
use engine::window::DIMS;
use engine::components::*;
use engine::systems::scene_builder::SceneBuilder;
use nalgebra_glm::{vec3, vec3_to_vec4, Quat, quat, quat_angle_axis, quat_look_at, quat_yaw, quat_identity};
use nalgebra_glm as glm;

fn main() {
    env_logger::init();

    let asset_folder = "assets";
    let gltf = load_gltf(asset_folder, "cube.gltf")
        .expect("failed to load gltf");

    let event_loop = winit::event_loop::EventLoop::new();
    let window_builder = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::LogicalSize::new(1.0, 1.0))
        .with_inner_size(winit::dpi::LogicalSize::new(
            DIMS.width,
            DIMS.height,
        ))
        .with_title("colour-uniform".to_string());

    let backend = create_backend(window_builder, &event_loop);

    let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

    let cube_mesh = StaticMeshData {
        id: 0,
        indices: mesh_data.indices.clone(),
        vertices: mesh_data.vertices.clone(),
    };

    let mut scene = Scene::default();
    scene.mesh_data.push(cube_mesh);

    let mut world = World::new();

    let mut init = DispatcherBuilder::new()
        .with(SceneBuilder, "scene_builder", &[])
        .build();

    init.setup(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(SceneBuilder, "scene_builder", &[])
        .build();

    dispatcher.setup(&mut world);

    world.insert(scene);

    let floor = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(0.0, 0.0, 0.0)))
        .with(Rotation(quat_identity()))
        .with(Scale(glm::vec3(10.0, 1.0, 10.0)))
        .build();

    let camera = world.create_entity()
        .with(Position(vec3(0.0, 2.0, 6.0)))
        .with(Rotation(quat_look_at(&vec3(0.0, 2.0, -7.0), &vec3(0.0, 1.0, 0.0))))
        .with(Camera)
        .build();

    let light = world.create_entity()
        .with(PointLight(40.0))
        .with(Position(vec3(1.5, 7.0, 4.0)))
        .build();

    let left_block = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(-2.0, 1.0, -2.0)))
        .with(Scale(vec3(1.0, 1.0, 1.0)))
        .with(Rotation(quat_identity()))
        .build();

    let tall_block = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(0.0, 2.0, 1.0)))
        .with(Scale(vec3(0.5, 3.0, 0.5)))
        .with(Rotation(quat_identity()))
        .build();

    let right_block = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(3.0, 1.0, -4.0)))
        .with(Scale(vec3(1.0, 2.0, 1.0)))
        .with(Rotation(quat_identity()))
        .build();

    init.dispatch(&world);

    let mut renderer = unsafe {
        Pathtracer::new(backend,  &world.fetch::<Scene>())
    };

    event_loop.run(move|event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                #[allow(unused_variables)]
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
                    },
                    _ => (),
                }
            },
            winit::event::Event::RedrawRequested(_) => {
                println!("RedrawRequested");
                dispatcher.dispatch(&world);
                renderer.render(&world.fetch::<Scene>());
            },
            winit::event::Event::MainEventsCleared => {
                renderer.backend.window.request_redraw();
                println!("EventsCleared");
            }
            _ => (),
        }
    });
}