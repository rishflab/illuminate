use crate::components::*;
use crate::input::MoveCommand;
use crate::resources::*;
use specs::prelude::*;
use glm::vec3;
use std::borrow::Borrow;
use glm::vec1;

pub struct FlyingMovement;

impl<'a> System<'a> for FlyingMovement {
    type SystemData = (
        Read<'a, MoveCommand>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, Player>,
        WriteStorage <'a, Position>,
        WriteStorage <'a, Rotation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (move_command, delta_time, players, mut positions, mut rotations) = data;

        let move_speed = 10.0;
        let delta_t = delta_time.0.as_secs_f32();
        let roll_sens = 0.5;

        (&players, &mut positions, &mut rotations).join()
            .for_each(|(_, position, rotation)|{
                match *move_command {
                    MoveCommand::Forward => {
                        let temp: glm::Quat = rotation.0;
                        let delta = delta_t * move_speed * glm::quat_cross_vec(&temp, &vec3(0.0, 0.0, -1.0));
                        println!("delta: {:?}, {:?} ", delta, glm::length(&delta));
                        position.0 = position.0 + (delta);
                    },
                    MoveCommand::Back => {
                        let temp: glm::Quat = rotation.0;
                        let delta = move_speed * glm::quat_cross_vec(&temp, &vec3(0.0, 0.0, 1.0));
                        position.0 = position.0 + (delta);
                    },
                    MoveCommand::Look(x, y) => {
                        let pitch: glm::Quat = glm::quat_angle_axis(-glm::radians(&vec1(y as f32)).x, &vec3(1.0, 0.0, 0.0));
                        let roll: glm::Quat = glm::quat_angle_axis(glm::radians(&vec1(roll_sens * x as f32)).x, &vec3(0.0, 0.0, 1.0));
                        rotation.0 =  roll * rotation.0 * pitch;
                    }
                    _ => (),
                }
            })
    }
}
