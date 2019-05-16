use winit;

pub struct InputState {
    pub events_loop: winit::EventsLoop,
}

pub enum Command {
    MoveLeft,
    MoveRight,
    Close,
}

impl InputState{

    pub fn new() -> InputState {
        let events_loop = winit::EventsLoop::new();

        InputState {
            events_loop: events_loop,
        }
    }

    pub fn process_raw_input(&mut self) -> Option<Command> {

        let mut next = None;

        self.events_loop.poll_events(|event| {
            if let winit::Event::WindowEvent { event, .. } = event {
                #[allow(unused_variables)]
                match event {
                    winit::WindowEvent::KeyboardInput {
                        input:
                        winit::KeyboardInput {
                            virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    }
                    | winit::WindowEvent::CloseRequested => next = Some(Command::Close),
                    winit::WindowEvent::KeyboardInput {
                        input:
                        winit::KeyboardInput {
                            virtual_keycode: Some(winit::VirtualKeyCode::Left),
                            ..
                        },
                        ..
                    } => next = Some(Command::MoveLeft),
                    winit::WindowEvent::KeyboardInput {
                        input:
                        winit::KeyboardInput {
                            virtual_keycode: Some(winit::VirtualKeyCode::Right),
                            ..
                        },
                        ..
                    } => next = Some(Command::MoveRight),
                    _ => (),
                }
            }
        });

        next

    }
}