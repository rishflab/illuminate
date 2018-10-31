//use cgmath::{Point2};
//use specs::prelude::*;
//use smallvec::SmallVec;
//use collision::primitive::Rectangle;
//
//
//#[derive(Clone, Debug)]
//struct SpaceShip<T> {
//    position: Point2<T>,
//    rotation: T,
//    width: T,
//    height: T,
//    thrust: T,
//    rotation_center: Point2<T>,
//    hitbox: Rectangle<T>,
//}
//
//impl SpaceShip<T> {
//    fn top_speed(&self) -> T {
//        self.thrust/(0.2 * self.width)
//    }
//
//
//}
//
//impl Component for SpaceShip<T> {
//    type Storage = VecStorage<Self>;
//}
//
//struct Thruster {
//    thrust: f32,
//    collider: SmallVec<Entity>,
//}
//
//impl Component for Thruster {
//    type Storage = VecStorage<Self>;
//}
//
//struct AddThruster {}
//
//struct AddThrusterHandler;
//
//impl<'a> System<'a> for AddThrusterHandler {
//
//    type SystemData = (ReadExpect<'a, AddThruster>, WriteStorage<'a, SpaceShip<T>>);
//
//    fn run(&mut self, (command, mut next_moves):Self::SystemData) {
//
//    }
//
//}
//
//struct DestroyThruster;
//
//struct DestroyThrusterHandler;
//
//impl<'a> System<'a> for AddThrusterHandler {
//
//    type SystemData = (ReadExpect<'a, DestroyThruster>, WriteStorage<'a, SpaceShip<T>>);
//
//    fn run(&mut self, (command, mut next_moves):Self::SystemData) {
//
//    }
//
//}
//
//
