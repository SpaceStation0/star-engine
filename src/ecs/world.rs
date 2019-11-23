use specs::prelude::*;

#[derive(Default)]
pub struct World {
    world: specs::World
}

impl World {
    pub fn new() -> World {
        Default::default()
    }
    pub fn register_component() {}
    pub fn register_script_component() {}
}