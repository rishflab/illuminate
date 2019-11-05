use crate::components::*;
use crate::input::MoveCommand;
use specs::prelude::*;
use glm::vec3;

pub struct PlayerMovement;

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (
        Read<'a, MoveCommand>,
        ReadStorage<'a, Player>,
        WriteStorage <'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (move_command, players, mut transforms) = data;
        (&players, &mut transforms).join()
            .for_each(|(_, transform)|{
                match *move_command {
                    MoveCommand::MoveLeft => {
                        transform.position = transform.position + vec3(-0.1, 0.0, 0.0);
                        //self.look_at = self.look_at + glm::vec3(-0.1, 0.0, 0.0);
                    },
                    MoveCommand::MoveRight => {
                        transform.position = transform.position + vec3(0.1, 0.0, 0.0);
                        //self.look_at = self.look_at + glm::vec3(0.1, 0.0, 0.0);
                    },
                    MoveCommand::None => (),
                }
            })

    }
}