#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use blackhole::input::UserInput;
use blackhole::renderer::fast_renderer::{HalState};
use blackhole::renderer::mesh::{Triangle};
use blackhole::renderer::window::WinitState;


use specs::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct LocalState {
    pub frame_width: f64,
    pub frame_height: f64,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

impl LocalState {
    pub fn update_from_input(&mut self, input: UserInput) {
        if let Some(frame_size) = input.new_frame_size {
            self.frame_width = frame_size.0;
            self.frame_height = frame_size.1;
        }
        if let Some(position) = input.new_mouse_position {
            self.mouse_x = position.0;
            self.mouse_y = position.1;
        }
    }
}

fn main() {

    simple_logger::init().unwrap();

    let mut world = World::new();
    world.register::<Triangle>();

    world.create_entity().with(Triangle::default()).build();

    let mut winit_state = WinitState::default();

//    let mut hal_state = match HalState::new(&winit_state.window) {
//        Ok(state) => state,
//        Err(e) => panic!(e),
//    };

    HalState::new(&winit_state.window);

    let (frame_width, frame_height) = winit_state
        .window
        .get_inner_size()
        .map(|logical| logical.into())
        .unwrap_or((0.0, 0.0));

    let mut local_state = LocalState {
        frame_width,
        frame_height,
        mouse_x: 0.0,
        mouse_y: 0.0,
    };

//    loop {
//        let inputs = UserInput::poll_events_loop(&mut winit_state.events_loop);
//        if inputs.end_requested {
//            break;
//        }
//        if inputs.new_frame_size.is_some() {
//            debug!("Window changed size, restarting HalState...");
//            //drop(hal_state);
//            hal_state = match HalState::new(&winit_state.window) {
//                Ok(state) => state,
//                Err(e) => panic!(e),
//            };
//        }
//        local_state.update_from_input(inputs);
//        if let Err(e) = do_the_render(&mut hal_state, &local_state) {
//            error!("Rendering Error: {:?}", e);
//            debug!("Auto-restarting HalState...");
//            //drop(hal_state);
//            hal_state = match HalState::new(&winit_state.window) {
//                Ok(state) => state,
//                Err(e) => panic!(e),
//            };
//        }

        //hal_state.render_world(&world);
//    }
}