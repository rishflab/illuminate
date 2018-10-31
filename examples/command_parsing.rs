extern crate specs;
extern crate shrev;
//extern crate teleport;

use specs::prelude::*;
use shrev::*;
//use MovePosition;

#[derive(Clone, Debug)]
struct MoveCommand {
    entity: Entity,
    translate: f32,
}

impl Component for MoveCommand {
     type Storage = VecStorage<Self>;
}


struct MoveCommand2(f32);

impl Component for MoveCommand2 {
    type Storage = VecStorage<Self>;
}

struct Position(f32);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct CommandBuffer {
    move_commands: Vec<MoveCommand>,
}

impl CommandBuffer {
    fn new() -> Self {
        CommandBuffer {
            move_commands: Vec::new(),
        }
    }

    fn add_move_command(&mut self, new_command: MoveCommand) {
        self.move_commands.push(new_command);
    }
}

impl Default for CommandBuffer {
      fn default() -> Self {
        Self::new()
    }
}

struct MovePosition;

impl<'a> System<'a> for MovePosition {
    type SystemData = (ReadStorage<'a, MoveCommand>, WriteStorage<'a, Position>);

    fn run(&mut self, (move_commands, mut positions): Self::SystemData) {

        for (move_command, position) in (&move_commands, &mut positions).join() {

            if position.0 + move_command.translate < 10.0 {

                position.0 += move_command.translate;
            }

        }
    }
}

struct CommandAllocator;

//impl Default for CommandAllocator {
//      fn default() -> Self {
//        Self::new()
//    }
//}

impl<'a> System<'a> for CommandAllocator {

    type SystemData = (Read<'a, CommandBuffer>, WriteStorage<'a, MoveCommand>);

    fn run(&mut self, (command_buffer, mut next_moves):Self::SystemData) {

        //for command in command_buffer.move_commands.iter() {

        println!("{}", command_buffer.move_commands.len());

            let command = &command_buffer.move_commands[0];

            let entity = command.entity;

            let entity_move_command = next_moves.get_mut(entity).unwrap();

            *entity_move_command = command.clone();

        //}
    }

}



fn main() {


    let mut world = World::new();
    world.register::<Position>();
    world.register::<MoveCommand>();
    world.add_resource(CommandBuffer::new());
    
//
//    let player1 = world.create_entity()
//        .with(Position(10.0))
//        .with(MoveCommand2(0.0))
//        .build();
//
//    {
//        let mut cmd_buf = world.write_resource::<CommandBuffer>();
//
//        cmd_buf.add_move_command(MoveCommand { entity: player1.clone(), translate: 3.2 });
//    }
//
//    let mut dispatcher = DispatcherBuilder::new()
//        .with(CommandAllocator, "command_allocator", &[])
//        .with(MovePosition, "move_position", &["command_allocator"])
//        .build();
//
//    dispatcher.dispatch(&mut world.res);

}