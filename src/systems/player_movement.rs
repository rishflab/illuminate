use cgmath::{Point2, Rad, Rotation2, Transform};
use shrev::EventChannel;
use specs::prelude::{
    System, Builder, DispatcherBuilder, Dispatcher, Component, RunNow, World, VecStorage,
    WriteStorage, ReadStorage
};



#[derive(Clone, Debug)]
struct Position(f32);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct SpaceShip {



}

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
//}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn thrust() {

        let mut world = World::new();

        let mut collision_system
        = BasicCollisionSystem2::<f32, BodyPose2<f32>, ()>::new()
            .with_broad_phase(BroadBruteForce2::default())
            .with_narrow_phase(GJK2::new());

        //collision_system.setup(&mut world.res);

        let mut reader_1 = world
            .write_resource::<EventChannel<ContactEvent2<f32>>>()
            .register_reader();

        world
            .create_entity()
            .with(CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(1.0, 1.0).into(),
            ))
            .with(BodyPose2::<f32>::new(
                Point2::new(0.0, 0.0),
                Rotation2::from_angle(Rad(0.0)),
            ))
            .build();

        world
            .create_entity()
            .with(CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(1.0, 1.0).into(),
            ))
            .with(BodyPose2::<f32>::new(
                Point2::new(0.98, 0.98),
                Rotation2::from_angle(Rad(0.0)),
            ))
            .build();


        collision_system.run_now(&world.res);


        println!(
            "Contacts: {:?}",
            world
                .read_resource::<EventChannel<ContactEvent2<f32>>>()
                .read(&mut reader_1)
                .collect::<Vec<_>>()
        );
    }
}


