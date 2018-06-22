extern crate specs;

use specs::{Component, FlaggedStorage, Join, System, VecStorage, WriteStorage};
use specs::prelude::*;

pub struct Comp(u32);

impl Component for Comp {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

pub struct CompSystem{
    // These keep track of where you left off in the event channel.
    modified_id: ReaderId<ModifiedFlag>,
    inserted_id: ReaderId<InsertedFlag>,

    // The bitsets you want to populate with modification/insertion events.
    modified: BitSet,
    inserted: BitSet,
}

impl<'a> System<'a> for CompSystem {

    type SystemData = (Entities<'a>, WriteStorage<'a, Comp>);

    fn run(&mut self, (entities, mut comps): Self::SystemData) {
        // We want to clear the bitset first so we don't have left over events
        // from the last frame.
        //
        // However, if you want to accumulate changes over a couple frames then you
        // can only clear it when necessary. (This might be useful if you have some
        // sort of "tick" system in your game and only want to do operations every
        // 1/4th of a second or something)
        //
        // It is not okay to only read the events in an interval though as that could
        // leave behind events which would end up growing the event ring buffer to
        // extreme sizes.
        self.modified.clear();
        self.inserted.clear();


        // This will only include inserted components from last read, note that this
        // will not include `insert` calls if there already was a pre-existing component.
        comps.populate_inserted(&mut self.inserted_id, &mut self.inserted);

        // Iterates over all components like normal.
        for comp in (&comps).join() {
            // ...
        }





    }

}

fn main() {

    let mut world = World::new();
    world.register::<Comp>();

    let ent = world.create_entity().with(Comp(5)).build();

    let mut dispatcher = DispatcherBuilder::new().with(CompSystem, "comp_system", &[]).build();

    dispatcher.dispatch(&world.res);

}