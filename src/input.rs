use winit::{
    event::{Event, WindowEvent, DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

pub struct InputState {
    pub event_loop: EventLoop<()>,
}

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

impl InputState{

    pub fn new() -> InputState {
        let event_loop = EventLoop::new();
        InputState {
            event_loop: event_loop,
        }

    }
//
//    pub fn process_raw_input(&self) -> Option<Command> {
//
//        self.event_loop.run(  move|event, _, _ /*control_flow*/| {
//            let mut next = None;
//            //*control_flow = winit::event_loop::ControlFlow::Wait;
//            print!("event ");
//            match event {
//                Event::WindowEvent{event: window_event, ..} => {
//                    match window_event {
//                        WindowEvent::KeyboardInput {
//                            input:
//                            KeyboardInput {
//                                virtual_keycode: Some(VirtualKeyCode::Escape),
//                                ..
//                            },
//                            ..
//                        } => next = Some(Command::Close),
//                        WindowEvent::CloseRequested => next = Some(Command::Close),
////                        winit::WindowEvent::KeyboardInput {
////                            input:
////                            winit::KeyboardInput {
////                                virtual_keycode: Some(winit::VirtualKeyCode::Left),
////                                ..
////                            },
////                            ..
////                        } => next = Some(Command::MoveCmd(MoveCommand::Left)),
////                        winit::WindowEvent::KeyboardInput {
////                            input:
////                            winit::KeyboardInput {
////                                virtual_keycode: Some(winit::VirtualKeyCode::Right),
////                                ..
////                            },
////                            ..
////                        } => next = Some(Command::MoveCmd(MoveCommand::Right)),
////                        winit::WindowEvent::KeyboardInput {
////                            input:
////                            winit::KeyboardInput {
////                                virtual_keycode: Some(winit::VirtualKeyCode::Up),
////                                state: winit::ElementState::Pressed,
////                                ..
////                            },
////                            ..
////                        } => next = Some(Command::MoveCmd(MoveCommand::Forward)),
////                        winit::WindowEvent::KeyboardInput {
////                            input:
////                            winit::KeyboardInput {
////                                virtual_keycode: Some(winit::VirtualKeyCode::Down),
////                                state: winit::ElementState::Pressed,
////                                ..
////                            },
////                            ..
////                        } => next = Some(Command::MoveCmd(MoveCommand::Back)),
//                        _ => (),
//                    }
//                },
//                Event::DeviceEvent {event: device_event, ..} => {
//                    match device_event {
////                        winit::DeviceEvent::MouseMotion {delta} => {
////                            next = Some(Command::MoveCmd(MoveCommand::Look(delta.0, delta.1)))
////                        },
//                        DeviceEvent::Key(keyboard_input) => {
//                            match keyboard_input {
//                                KeyboardInput {
//                                    virtual_keycode: Some(VirtualKeyCode::Up),
//                                    state: ElementState::Pressed,
//                                    ..
//                                } => next = Some(Command::MoveCmd(MoveCommand::Forward)),
//                                _ => (),
//                            }
//                        }
//                        _ => (),
//                    }
//                },
//                _ => (),
//            }
//        });
//
//        //println!("{:?}", next);
//        //next
//        None
//
//    }
}