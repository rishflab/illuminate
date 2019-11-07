use winit;

pub struct InputState {
    pub events_loop: winit::EventsLoop,
}

pub enum Command {
    MoveCmd(MoveCommand),
    Close,
}
pub enum MoveCommand {
    Left,
    Right,
    Forward,
    Back,
    Look(f64, f64),
    None,
}

impl Default for MoveCommand {
    fn default() -> Self{
        MoveCommand::None
    }
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
            match event {
                winit::Event::WindowEvent{event: window_event, ..} => {
                    match window_event {
                        winit::WindowEvent::KeyboardInput {
                            input:
                            winit::KeyboardInput {
                                virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => next = Some(Command::Close),
                        winit::WindowEvent::CloseRequested => next = Some(Command::Close),
                        winit::WindowEvent::KeyboardInput {
                            input:
                            winit::KeyboardInput {
                                virtual_keycode: Some(winit::VirtualKeyCode::Left),
                                ..
                            },
                            ..
                        } => next = Some(Command::MoveCmd(MoveCommand::Left)),
                        winit::WindowEvent::KeyboardInput {
                            input:
                            winit::KeyboardInput {
                                virtual_keycode: Some(winit::VirtualKeyCode::Right),
                                ..
                            },
                            ..
                        } => next = Some(Command::MoveCmd(MoveCommand::Right)),
                        winit::WindowEvent::KeyboardInput {
                            input:
                            winit::KeyboardInput {
                                virtual_keycode: Some(winit::VirtualKeyCode::Up),
                                ..
                            },
                            ..
                        } => next = Some(Command::MoveCmd(MoveCommand::Left)),
                        winit::WindowEvent::KeyboardInput {
                            input:
                            winit::KeyboardInput {
                                virtual_keycode: Some(winit::VirtualKeyCode::Down),
                                ..
                            },
                            ..
                        } => next = Some(Command::MoveCmd(MoveCommand::Back)),
                        _ => (),
                    }
                },
                winit::Event::DeviceEvent {event: device_event, ..} => {
                    match device_event {
                        winit::DeviceEvent::MouseMotion {delta} => {
                            next = Some(Command::MoveCmd(MoveCommand::Look(delta.0, delta.1)))
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        });


        next

    }
}