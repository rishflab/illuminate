extern crate blackhole;

use blackhole::renderer::RendererState;
use blackhole::renderer::window::WindowState;
use blackhole::renderer::backend::{BackendState, create_backend};

fn main() {
    env_logger::init();

    let mut window = WindowState::new();
    let (backend, _instance) = create_backend(&mut window);

    let mut renderer_state = unsafe { RendererState::new(backend, window) };

    renderer_state.mainloop();
}