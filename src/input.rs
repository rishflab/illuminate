use winit::{
    event::{Event, WindowEvent, DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;


#[derive(Debug)]
pub enum Command {
    MoveCmd(MoveCommand),
    Close,
}

#[derive(Debug)]
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


pub fn process_window_event(event: &WindowEvent) -> MoveCommand {
    match event {
        WindowEvent::KeyboardInput {
            input:
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Left),
                ..
            },
            ..
        } => MoveCommand::Left,
        WindowEvent::KeyboardInput {
            input:
            KeyboardInput{
                virtual_keycode: Some(VirtualKeyCode::Right),
                ..
            },
            ..
        } => MoveCommand::Right,
        WindowEvent::KeyboardInput {
            input:
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Up),
                state: ElementState::Pressed,
                ..
            },
            ..
        } => MoveCommand::Forward,
        WindowEvent::KeyboardInput {
            input:
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Down),
                state: ElementState::Pressed,
                ..
            },
            ..
        } => MoveCommand::Back,
        _ => MoveCommand::None,
    }
}



pub fn process_raw_input<T>(event: &Event<T>) -> Option<Command> {
    match event {
        Event::WindowEvent { event: window_event, .. } => {
            match window_event {
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => Some(Command::Close),
                WindowEvent::CloseRequested => Some(Command::Close),
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        ..
                    },
                    ..
                } => Some(Command::MoveCmd(MoveCommand::Left)),
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        ..
                    },
                    ..
                } => Some(Command::MoveCmd(MoveCommand::Right)),
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                } => Some(Command::MoveCmd(MoveCommand::Forward)),
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                } => Some(Command::MoveCmd(MoveCommand::Back)),
                _ => None,
            }
        },
        Event::DeviceEvent { event: device_event, .. } => {
            match device_event {
                DeviceEvent::MouseMotion { delta } => {
                    Some(Command::MoveCmd(MoveCommand::Look(delta.0, delta.1)))
                },
                DeviceEvent::Key(keyboard_input) => {
                    match keyboard_input {
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Up),
                            state: ElementState::Pressed,
                            ..
                        } => Some(Command::MoveCmd(MoveCommand::Forward)),
                        _ => None,
                    }
                }
                _ => None,
            }
        },
        _ => None,
    }
}
