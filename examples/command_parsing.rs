//extern crate specs;
//extern crate shrev;
////extern crate teleport;
//
//use specs::prelude::*;
////use shrev::*;
////use MovePosition;
//
//#[derive(Clone, Debug)]
//struct MoveCommand {
//    entity: Entity,
//    translate: f32,
//}
//
//impl Component for MoveCommand {
//     type Storage = VecStorage<Self>;
//}
//
//#[derive(Clone, Debug)]
//struct PlayerMoved(f32);
//
//impl Component for PlayerMoved {
//    type Storage = VecStorage<Self>;
//}
//
//#[derive(Clone, Debug)]
//struct Position(f32);
//
//impl Component for Position {
//    type Storage = VecStorage<Self>;
//}
//
//struct CommandBuffer {
//    move_commands: Vec<MoveCommand>,
//}
//
//impl CommandBuffer {
//    fn new() -> Self {
//        CommandBuffer {
//            move_commands: Vec::new(),
//        }
//    }
//
//    fn add_move_command(&mut self, new_command: MoveCommand) {
//        self.move_commands.push(new_command);
//    }
//}
//
//impl Default for CommandBuffer {
//      fn default() -> Self {
//          let a = Self::new();
//          //a.add_move_command(MoveCommand{4.3});
//          a
//    }
//}
//
//struct MovePosition;
//
//impl<'a> System<'a> for MovePosition {
//    type SystemData = (ReadStorage<'a, MoveCommand>, WriteStorage<'a, Position>);
//
//    fn run(&mut self, (move_commands, mut positions): Self::SystemData) {
//
//        for (move_command, position) in (&move_commands, &mut positions).join() {
//
//            if position.0 + move_command.translate < 10.0 {
//
//                position.0 += move_command.translate;
//            }
//
//        }
//    }
//
//struct CommandAllocator;
//
//
//impl<'a> System<'a> for CommandAllocator {
//
//    type SystemData = (ReadExpect<'a, CommandBuffer>, WriteStorage<'a, PlayerMoved>);
//
//    fn run(&mut self, (command_buffer, mut next_moves):Self::SystemData) {
//
//    //for command in command_buffer.move_commands.iter() {
//
//    println ! ("{}", command_buffer.move_commands.len());
//
//    let command = & command_buffer.move_commands[0];
//
//
//    }
//
//}
//
//
//
//fn main() {
//
//    let mut world = World::new();
//    world.register::<Position>();
//    world.register::<PlayerMoved>();
//    world.register::<MoveCommand>();
//
//    world.add_resource(CommandBuffer::new());
//
////
////    let player1 = world.create_entity()
////        .with(Position(10.0))
////        .with(MoveCommand2(0.0))
////        .build();
////
////    {
////        let mut cmd_buf = world.write_resource::<CommandBuffer>();
////
////        cmd_buf.add_move_command(MoveCommand { entity: player1.clone(), translate: 3.2 });
////    }
////
////    let mut dispatcher = DispatcherBuilder::new()
////        .with(CommandAllocator, "command_allocator", &[])
////        .with(MovePosition, "move_position", &["command_allocator"])
////        .build();
////
//
//    world.add_resource(CommandBuffer::default());
//
//    //world.add_resource(12.0);
//
//    let player1 = world.create_entity()
//        .with(Position(10.0))
//        .with(PlayerMoved(1.2))
//        .build();
//
//
//    {
//        let mut cmd_buf = world.write_resource::<CommandBuffer>();
//        cmd_buf.add_move_command(MoveCommand{entity: player1, translate: 3.2});
//    }
//
//
//    {
//        let res = world.read_resource::<CommandBuffer>();
//        println!("{:?}", res.move_commands);
//    }
//
//    println!("{:?}", world.read_resource::<CommandBuffer>().move_commands);
//
//
//
//    let mut dispatcher = DispatcherBuilder::new()
//
//
//        .with(CommandAllocator, "command_allocator", &[])
//         //.with(MovePosition, "move_position", &["command_allocator"])
//        .build();
//    dispatcher.setup(&mut world.res);
//    dispatcher.dispatch(&world.res);
//
//
//}