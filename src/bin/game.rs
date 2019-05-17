extern crate blackhole;

use blackhole::renderer::RendererState;
use blackhole::renderer::window::WindowState;
use blackhole::renderer::backend::{create_backend};
use blackhole::input::{InputState, Command};
use blackhole::renderer::scene::Scene;

fn main() {
    env_logger::init();

    let mut window = WindowState::new();
    let mut input = InputState::new();
    let (backend, _instance) = create_backend(&mut window, &mut input);

    let mut renderer_state = unsafe { RendererState::new(backend, window) };

    let mut running = true;

    let mut scene = Scene::new();

    while running {

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


        renderer_state.render(&scene);
    }
}