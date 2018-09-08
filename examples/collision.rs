extern crate cgmath;
extern crate rhusics_core;
extern crate rhusics_ecs;
extern crate shrev;
extern crate specs;

use cgmath::{Point2, Rad, Rotation2, Transform};
use shrev::EventChannel;
use specs::prelude::{DispatcherBuilder, Builder, RunNow, World};

use rhusics_core::Pose;
use rhusics_ecs::collide2d::{
    BasicCollisionSystem2, BodyPose2, BroadBruteForce2, CollisionMode, CollisionShape2,
    CollisionStrategy, ContactEvent2, GJK2, Rectangle,
};

pub fn main() {
    let mut world = World::new();

    let mut system = BasicCollisionSystem2::<f32, BodyPose2<f32>, ()>::new()

        .with_broad_phase(BroadBruteForce2::default())
        .with_narrow_phase(GJK2::new());
    system.setup(&mut world.res);

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

    //system.run_now(&world.res);

    let mut dispatcher = DispatcherBuilder::new()
        .with(system, "collision system", &[])
        .build();

    dispatcher.setup(&mut world.res);

    dispatcher.dispatch(&world.res);

    println!(
        "Contacts: {:?}",
        world
            .read_resource::<EventChannel<ContactEvent2<f32>>>()
            .read(&mut reader_1)
            .collect::<Vec<_>>()
    );
}