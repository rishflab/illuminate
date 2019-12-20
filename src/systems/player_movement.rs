use crate::components::*;
use crate::input::{KeyboardState, MouseTravel};
use crate::resources::*;
use specs::prelude::*;
use glm::vec3;
use std::borrow::Borrow;
use glm::vec1;
use winit::event::VirtualKeyCode;

pub struct FlyingMovement;

impl<'a> System<'a> for FlyingMovement {
    type SystemData = (
        Read<'a, KeyboardState>,
        Write<'a, MouseTravel>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, Player>,
        WriteStorage <'a, Position>,
        WriteStorage <'a, Rotation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (keyboard, mut mouse, delta_time, players, mut positions, mut rotations) = data;

        let move_speed = 10.0;
        let delta_t = delta_time.0.as_secs_f32();
        let roll_sens = 0.1;

        (&players, &mut positions, &mut rotations).join()
            .for_each(|(_, position, rotation)|{
                if keyboard.is_key_pressed(VirtualKeyCode::Up) {
                    let temp: glm::Quat = rotation.0;
                    let delta = move_speed * delta_t * glm::quat_cross_vec(&temp, &vec3(0.0, 0.0, -1.0));
//                    println!("delta: {:?}, {:?} ", delta, glm::length(&delta));
                    position.0 = position.0 + (delta);
                }
                if keyboard.is_key_pressed(VirtualKeyCode::Down) {
                    let temp: glm::Quat = rotation.0;
                    let delta = move_speed * delta_t * glm::quat_cross_vec(&temp, &vec3(0.0, 0.0, 1.0));
                    position.0 = position.0 + (delta);
                }
                let pitch: glm::Quat = glm::quat_angle_axis(-glm::radians(&vec1(roll_sens * mouse.y as f32)).x, &vec3(1.0, 0.0, 0.0));
                let roll: glm::Quat = glm::quat_angle_axis(glm::radians(&vec1(roll_sens * mouse.x as f32)).x, &vec3(0.0, 0.0, 1.0));
                rotation.0 = roll * rotation.0 * pitch;
                mouse.reset();
            })
    }
}

pub struct FlyingFPSMovement;

impl<'a> System<'a> for FlyingFPSMovement {
    type SystemData = (
        Read<'a, KeyboardState>,
        Write<'a, MouseTravel>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Rotation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (keyboard, mut mouse, delta_time, players, mut positions, mut rotations) = data;

        let move_speed = 10.0;
        let delta_t = delta_time.0.as_secs_f32();
        let roll_sens = 0.1;

        (&players, &mut positions, &mut rotations).join()
            .for_each(|(_, position, rotation)|{
                if keyboard.is_key_pressed(VirtualKeyCode::Up) {
                    let temp: glm::Quat = rotation.0;
                    let delta = move_speed * delta_t * glm::quat_cross_vec(&temp, &vec3(0.0, 0.0, -1.0));
                    position.0 = position.0 + (delta);
                }
                if keyboard.is_key_pressed(VirtualKeyCode::Down) {
                    let temp: glm::Quat = rotation.0;
                    let delta = move_speed * delta_t * glm::quat_cross_vec(&temp, &vec3(0.0, 0.0, 1.0));
                    position.0 = position.0 + (delta);
                }
                if keyboard.is_key_pressed(VirtualKeyCode::Left) {
                    let temp: glm::Quat = rotation.0;
                    let delta = move_speed * delta_t * glm::quat_cross_vec(&temp, &vec3(-1.0, 0.0, 0.0));
                    position.0 = position.0 + (delta);
                }
                if keyboard.is_key_pressed(VirtualKeyCode::Right) {
                    let temp: glm::Quat = rotation.0;
                    let delta = move_speed * delta_t * glm::quat_cross_vec(&temp, &vec3(1.0, 0.0, 0.0));
                    position.0 = position.0 + (delta);
                }
                let pitch: glm::Quat = glm::quat_angle_axis(glm::radians(&vec1(roll_sens * mouse.y as f32)).x, &vec3(-1.0, 0.0, 0.0));
                let yaw: glm::Quat = glm::quat_angle_axis(glm::radians(&vec1(roll_sens * mouse.x as f32)).x, &vec3(0.0, -1.0, 0.0));
                rotation.0 = yaw * rotation.0 * pitch;
                mouse.reset();
            })
    }
}