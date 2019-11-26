//! Star Engine is powered by a robust ECS system. This system
//! executes in multiple phases and is designed to be fast,
//! easily extendable, and endlessly configurable.
//! Everything that happens inside the game is controlled from here.
//!
//! Star Engine has a tick-based design. Each system
//! has an execution policy and is responsible for knowing when
//! to execute.
//!
//! Everything that happens occurs in the `Game` class. This
//! is the container for all sub-elements of the game. These
//! sub-elements include:
//! - The entity world
//! - The systems dispatcher
//! - The resource allocator
//! - The client handler
//! - The scripting engine
//!
//! Usually, the main function of an actual game
//! will be initializing everything in a `Game` struct
//! and then running it.
//!
//! Each sub-element of the `Game` struct is documented in
//! each of their respective folders.

use specs::{World, Dispatcher, DispatcherBuilder};
use specs::shred::ParSeq;
use super::script::PythonInterpreter;

pub mod event;
pub mod notifier;
pub mod network;

/// The main struct, from which all game execution
/// occurs. A container of all engine elements.
pub struct Game<'a, 'b, P, T> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    notify_dispatcher: ParSeq<P, T>,

}

pub struct GameBuilder<'a, 'b, P, T> {
    world: World,
    dispatcher: DispatcherBuilder<'a, 'b>,
    notify_dispatcher: Option<ParSeq<P, T>>
}

impl<'a, 'b, P, T> Game<'a, 'b, P, T> {
    pub fn new() -> GameBuilder<'a, 'b, P, T> {
        GameBuilder {
            world: World::new(),
            dispatcher: DispatcherBuilder::new(),
            notify_dispatcher: None
        }
    }

    //TODO: Add server options
    pub fn start_server(&mut self) {

    }

    pub fn tick(&mut self) {

    }

    pub fn status(&self) {

    }

    pub fn reboot(&mut self) {

    }
}

impl<'a, 'b, P, T> Game<'a, 'b, P, T> {
    async fn internal_tick(&mut self) -> Result<(), String> {
        Ok(())
    }

    // Initialize the global state
    fn initialize(&mut self) {

    }

    fn clear_world(&mut self) {
        let mut world = World::new();
        std::mem::swap(&mut self.world, &mut world);
        drop(world);
    }
}
