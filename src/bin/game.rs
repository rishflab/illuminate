extern crate blackhole;

use blackhole::renderer::pathtracer::Pathtracer;
use blackhole::window::WindowState;
use blackhole::renderer::core::backend::{create_backend};
use blackhole::input::{InputState, Command};
use blackhole::renderer::scene::Scene;
use blackhole::asset::{MeshData, load_gltf};

fn main() {
    env_logger::init();

    let mut scene = Scene::cube();

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
                        &scene.update_model_position(command);
                        println!("model location: {:?}", scene.model_pos);
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