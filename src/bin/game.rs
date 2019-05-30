extern crate blackhole;

use blackhole::renderer::RendererState;
use blackhole::renderer::window::WindowState;
use blackhole::renderer::backend::{create_backend};
use blackhole::input::{InputState, Command};
use blackhole::renderer::scene::Scene;
use blackhole::asset::{mesh_data_from_gltf, MeshData, load_gltf};

fn main() {
    env_logger::init();

    let asset_folder = "assets";
    let gltf = load_gltf(asset_folder, "Box.gltf").expect("failed to load gltf");
    let mesh_data = mesh_data_from_gltf(&gltf, asset_folder);

    let mut scene = Scene::new(mesh_data);

    let mut window = WindowState::new();
    let mut input = InputState::new();
    let (backend, _instance) = create_backend(&mut window, &mut input);

    let mut renderer_state = unsafe { RendererState::new(backend, window, &scene) };

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
                    },
                }
            },
            None => (),
        }

        let start = Instant::now();

        renderer_state.render(&scene);

        let duration = start.elapsed();

        println!("Frame time {:?}", duration);
    }
}