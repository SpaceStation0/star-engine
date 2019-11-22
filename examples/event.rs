extern crate star_engine;
use star_engine::ecs::shred_event::*;
use std::any::Any;

type PlayerID = u64;

struct Attack {
    damage: u32,
    from: PlayerID,
    to: PlayerID
}

impl Event for Attack {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

fn main() {

}