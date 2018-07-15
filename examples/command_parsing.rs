extern crate specs;
extern crate shrev;
extern crate teleport;

use specs::prelude::*;
use shrev::*;


struct MoveCommand {
    entity: Entity,
    translate: f32,
}

impl Component for MoveCommand {
     type Storage = VecStorage<Self>;
}

struct Position(f32);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct MovePosition {

}

struct CommandBuffer {
    move_commands: Vec<MoveCommand>,
}

impl<'a> System<'a> for MovePosition {
    type SystemData = (Entities<'a>, ReadStorage<'a, MoveCommand>, WriteStorage<'a, Position>);

    fn run(&self, (entities, move_commands, mut positions): Self::SystemData) {
        for (entity, move_command, position) in (&*entities, &move_commands, &mut positions).join() {
            if position.0 + move_command.translate < 10.0 {
                position.0 += move_command.translate;
            }

        }
    }
}

struct CommandAllocator;


impl<'a> System<'a> for CommandAllocator {
    type SystemData = (Read<>, ReadStorage<'a, MoveCommand>, )

}



fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<MoveCommand>();


    let player1 = world.create_entity()
        .with(Position(10.0))
        .build();


}