extern crate specs;
extern crate shrev;
extern crate teleport;

use specs::prelude::*;
use shrev::*;

#[derive(Debug)]
struct Position(f32);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct Id(u32);

impl Component for Id {
    type Storage = VecStorage<Self>;
}

struct AddDriver {
    position: Position,
}

impl Component for AddDriver {
    type Storage = VecStorage<Self>;
}

struct UpdateDriverPosition {
    position: Position,
}

impl Component for UpdateDriverPosition {
    type Storage = VecStorage<Self>;
}

struct Velocity(f32);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

struct PhysicsUpdate;

struct DriverUpdater;

impl<'a> System<'a> for DriverUpdater {

    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, UpdateDriverPosition>,
        ReadStorage<'a, Id>,
    );

    fn run(&mut self, (mut positions, commands, ids): Self::SystemData) {

//        let move_this_player_3m_north: Entity = ...;
//        let player_position: &mut PlayerPosition = player_position.get_mut(move_this_player_3m_north).unwrap();
//        player_position.move_3m_north();

//        for (command, id) in (&commands, &ids).par_join(){
//
//            let driver_position = positions.get_mut(command.id).unwrap();
//
//            if *driver_position.0 <= 10.0 {
//                 *driver_position = command.position.into();
//            }
//
//
//        }

    }
}




fn main() {
//
//    let mut event_channel = EventChannel::new();
//
//    let reader = event_channel.register_reader();

    let mut world = World::new();
    world.register::<Position>();
    world.register::<UpdateDriverPosition>();
    world.register::<Id>();

    let driver1 = world.create_entity().with(Position(3.0)).with(Id(1)).build();

    let driver2 = world.create_entity().with(Position(3.0)).with(Id(2)).build();

    let mut dispatcher = DispatcherBuilder::new().with(DriverUpdater, "driver_updater", &[]).build();

    dispatcher.dispatch(&world.res);


}