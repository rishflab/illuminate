extern crate blackhole;

use blackhole::renderer::pathtracer::Pathtracer;
use blackhole::window::WindowState;
use blackhole::renderer::core::backend::{create_backend};
use blackhole::input::{InputState, Command};
use blackhole::scene::Scene;

fn main() {
    env_logger::init();

    //let mut scene = Scene::two_cubes();

    //let mut scene = Scene::occluded_cubes();

    let mut scene = Scene::multiple_boxes();

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
                        &scene.lights[0].update_position(command);
                        //&scene.camera.update_position(command);
                        println!("camera location: {:?}", scene.lights[0].position);
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