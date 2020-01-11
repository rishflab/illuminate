use winit::{
    event::{Event, WindowEvent, DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use std::collections::HashSet;

#[derive(Debug)]
pub struct MouseTravel {
    pub x: f64,
    pub y: f64,
}

impl Default for MouseTravel {
    fn default() -> Self {
        MouseTravel {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl MouseTravel {
    pub fn add(&mut self, delta: (f64, f64)) {
        self.x += delta.0;
        self.y += delta.1;
    }
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}

pub struct KeyboardState {
    pressed_keys: HashSet<VirtualKeyCode>
}

impl Default for KeyboardState {
    fn default() -> Self {
        KeyboardState::new()
    }
}

impl KeyboardState {
    pub fn new() -> Self {
        KeyboardState {
            pressed_keys: HashSet::with_capacity(256)
        }
    }
    pub fn set_key(&mut self, key: VirtualKeyCode, pressed: bool) {
        if pressed {
            let _ = self.pressed_keys.insert(key);
        } else {
            let _ = self.pressed_keys.remove(&key);
        }
    }
    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
    pub fn process_device_input(&mut self, event: KeyboardInput) {
        match event {
            KeyboardInput {
                virtual_keycode: Some(key),
                state: ElementState::Pressed,
                ..
            } => self.set_key(key, true),
            KeyboardInput {
                virtual_keycode: Some(key),
                state: ElementState::Released,
                ..
            } => self.set_key(key, false),
            _ => (),
        }
    }
}