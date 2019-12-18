extern crate blackhole;

use blackhole::renderer::pathtracer::Pathtracer;
use blackhole::window::WindowState;
use blackhole::renderer::core::backend::{create_backend};
use blackhole::input::{InputState, Command, MoveCommand};
use blackhole::scene::{Scene};
use specs::prelude::*;
use blackhole::asset::{load_gltf, MeshData};
use blackhole::scene::mesh::{StaticMeshData, MeshInstance};
use blackhole::scene;
use blackhole::components::*;
use blackhole::systems::scene_builder::SceneBuilder;
use nalgebra_glm::{vec3, vec3_to_vec4, Quat, quat, quat_angle_axis, quat_look_at, quat_yaw, quat_identity};
use nalgebra_glm as glm;

fn main() {
    env_logger::init();

    let asset_folder = "assets";
    let gltf = load_gltf(asset_folder, "untitled.gltf")
        .expect("failed to load gltf");

    let mut window = WindowState::new();
    let mut input = InputState::new();
    let (backend, _instance) = create_backend(&mut window, &mut input);

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
        .with(SceneBuilder, "scene_builder", &[])
        .build();

    dispatcher.setup(&mut world);

    world.insert(Scene::default());

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
        Pathtracer::new(backend, window, &world.fetch::<Scene>())
    };

    let mut running = true;

    while running {
        use std::time::Instant;

//        match input.process_raw_input() {
//            Some(command) => {
//                match command {
//                    Command::Close => {
//                        running = false;
//                    },
//                    _ => (),
//                }
//            },
//            None => (),
//        }

        dispatcher.dispatch(&world);

        let start = Instant::now();

        renderer.render(&world.fetch::<Scene>());

        println!("aaaaaaa");

        let duration = start.elapsed();

        println!("Frame time {:?}", duration);
    }

}