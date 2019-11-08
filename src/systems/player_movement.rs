use crate::components::*;
use crate::input::MoveCommand;
use specs::prelude::*;
use glm::vec3;
use std::borrow::Borrow;
use glm::vec1;

pub struct FlyingMovement;

impl<'a> System<'a> for FlyingMovement {
    type SystemData = (
        Read<'a, MoveCommand>,
        ReadStorage<'a, Player>,
        WriteStorage <'a, Position>,
        WriteStorage <'a, Rotation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (move_command, players, mut positions, mut rotations) = data;

        let move_speed = 0.4;
        let roll_sens = 0.5;

        (&players, &mut positions, &mut rotations).join()
            .for_each(|(_, position, rotation)|{
                match *move_command {
                    MoveCommand::Forward => {
                        let temp: glm::Quat = rotation.0;
                        let delta = glm::quat_cross_vec(&temp, &vec3(0.0, 0.0, -1.0));
                        position.0 = position.0 + move_speed * delta;
                    },
                    MoveCommand::Back => {
                        let temp: glm::Quat = rotation.0;
                        let delta = glm::quat_cross_vec(&temp, &vec3(0.0, 0.0, 1.0));
                        position.0 = position.0 + move_speed * delta;
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
