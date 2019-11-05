use specs::prelude::*;

#[derive(Clone, Debug)]
struct MoveCommand {
    entity: Entity,
    translate: f32,
}

impl Component for MoveCommand {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
struct PlayerMoved(f32);

impl Component for PlayerMoved {
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
        let a = Self::new();
        a
    }
}

struct CommandAllocator;

impl<'a> System<'a> for CommandAllocator {

    type SystemData = (ReadExpect<'a, CommandBuffer>, WriteStorage<'a, PlayerMoved>);

    fn run(&mut self, (command_buffer, mut next_moves):Self::SystemData) {

        //for command in command_buffer.move_commands.iter() {

        println!("{}", command_buffer. move_commands.len());

        let command = &command_buffer.move_commands[0];

        println!("{:?}", command);

        let entity = command.entity;

        println!("{:?}", entity);


        let mut a = next_moves.get_mut(command.entity).unwrap();
        a.0 = command.translate;


    }

}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn allocate_move_command() {

        let mut world = World::new();
        world.register::<PlayerMoved>();
        world.register::<MoveCommand>();
        world.add_resource(CommandBuffer::default());


        let player1 = world.create_entity()
            .with(PlayerMoved(1.2))
            .build();


        {
            let mut cmd_buf = world.write_resource::<CommandBuffer>();
            cmd_buf.add_move_command(MoveCommand{entity: player1, translate: 3.2});
        }


        {
            let res = world.read_resource::<CommandBuffer>();
            println!("{:?}", res.move_commands);
        }

        println!("{:?}", world.read_resource::<CommandBuffer>().move_commands);



        let mut dispatcher = DispatcherBuilder::new()


            .with(CommandAllocator, "command_allocator", &[])

            .build();
        dispatcher.setup(&mut world.res);
        dispatcher.dispatch(&world.res);



    }




}
