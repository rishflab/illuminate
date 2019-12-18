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
use blackhole::resources::*;
use blackhole::systems::player_movement::FlyingMovement;
use blackhole::systems::scene_builder::SceneBuilder;
use nalgebra_glm::{vec3, vec3_to_vec4, Quat, quat, quat_angle_axis, quat_look_at, quat_yaw, quat_identity};
use nalgebra_glm as glm;
use blackhole::resources::DeltaTime;

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
    .with(FlyingMovement, "player_movement", &[])
    .with(SceneBuilder, "scene_builder", &["player_movement"])
    .build();

    dispatcher.setup(&mut world);

    world.insert(Scene::default());
    world.insert(MoveCommand::default());
    world.insert(DeltaTime::default());

    let floor = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(0.0, 0.0, 0.0)))
        .with(Rotation(quat_identity()))
        .with(Scale(glm::vec3(10.0, 1.0, 10.0)))
        .build();

    let player = world.create_entity()
        .with(Position(vec3(0.0, 2.0, 9.0)))
        .with(Rotation(quat_look_at(&vec3(0.0, 2.0, -7.0), &vec3(0.0, 1.0, 0.0))))
        .with(Player)
        .with(Camera)
        .build();

    let light = world.create_entity()
        .with(PointLight(60.0))
        .with(Position(vec3(1.5, 7.0, 4.0)))
        .build();

    init.dispatch(&world);

    let mut renderer = unsafe {
        Pathtracer::new(backend, window, &world.fetch::<Scene>())
    };

    let mut running = true;

//    while running {
//        use std::time::Instant;
//
//        let start = Instant::now();
//
//        {
//            let mut move_command = world.write_resource::<MoveCommand>();
//            *move_command = MoveCommand::None;
//        }
//
//        match input.process_raw_input() {
//            Some(command) => {
//                match command {
//                    Command::Close => {
//                        running = false;
//                    },
//                    Command::MoveCmd(next_move) => {
//                        let mut move_command = world.write_resource::<MoveCommand>();
//                        *move_command = next_move
//                    },
//                }
//            },
//            None => (),
//        }
//
//        dispatcher.dispatch(&world);
//
//        renderer.render(&world.fetch::<Scene>());
//
//        {
//            let duration = start.elapsed();
//            let mut delta_time = world.write_resource::<DeltaTime>();
//            delta_time.0 = duration;
//            println!("Frame time {:?}", duration);
//        }
//
//    }
}