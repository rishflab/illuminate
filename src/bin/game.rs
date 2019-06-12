extern crate blackhole;

use blackhole::renderer::pathtracer::Pathtracer;
use blackhole::window::WindowState;
use blackhole::renderer::core::backend::{create_backend};
use blackhole::input::{InputState, Command};
use blackhole::renderer::scene::Scene;
use blackhole::asset::{mesh_data_from_gltf, MeshData, load_gltf};

fn main() {
    env_logger::init();

    let asset_folder = "assets";
    let gltf = load_gltf(asset_folder, "untitled.gltf").expect("failed to load gltf");
    let mesh_data = mesh_data_from_gltf(&gltf, asset_folder);

    let mut scene = Scene::new(mesh_data);

    let mut window = WindowState::new();
    let mut input = InputState::new();
    let (backend, _instance) = create_backend(&mut window, &mut input);

    let mut renderer = unsafe {
        Pathtracer::new(backend, window, &scene)
    };

    let mut running = true;


    while running {

        use std::time::Instant;


        match input.process_raw_input() {
            Some(command) => {
                match command {
                    Command::Close => {
                        running = false;
                    },
                    _ => {
                        &scene.update_cube(command);
                        println!("cube locaiton: {:?}", scene.cube_pos);
                    },
                }
            },
            None => (),
        }

        let start = Instant::now();

        renderer.render(&scene);

        let duration = start.elapsed();

        println!("Frame time {:?}", duration);
    }
}