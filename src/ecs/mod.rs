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

use specs::{World, Dispatcher, DispatcherBuilder, System};
use crate::network::{Server, ClientMessageCodec};
use crate::script::system::InterpreterSystem;

pub mod event;
pub mod events;
pub mod notifier;
pub mod network;

/// The main struct, from which all game execution
/// occurs. A container of all engine elements.
pub struct Game<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    event_dispatcher: Dispatcher<'a, 'b>,
    interpreter_dispatcher: Vec<InterpreterSystem>,
    include_builtins: bool
}

pub struct GameBuilder<'a, 'b> {
    world: World,
    dispatcher: DispatcherBuilder<'a, 'b>,
    event_dispatcher: DispatcherBuilder<'a, 'b>,
    interpreter_dispatcher: Vec<InterpreterSystem>,
    include_builtins: bool
}


/// Updaters are payloads of data that can load themselves onto the world.
pub trait Updater {
    fn update_world(self, world: &mut World);
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new_builder() -> GameBuilder<'a, 'b> {
        GameBuilder::default()
    }

    //TODO: Add server options
    pub fn start_server<C>(&mut self, codec: C)
    where C: ClientMessageCodec + Send + 'static, C::Output: Default + Send + Sync {
        // Create a new server and serve it
        let server = Server::new(codec);
        server.start();

        // Add the codec as a resource
        self.world.add_resource(C::Output::default());
    }

    pub fn tick(&mut self) {
        for mut i in &mut self.interpreter_dispatcher {
            i.run(&self.world);
        }
    }

    pub fn status(&self) {
        unimplemented!()
    }

    pub fn reboot(&mut self) {
        unimplemented!()
    }
}

impl<'a, 'b> Game<'a, 'b> {
    async fn internal_tick(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn clear_world(&mut self) {
        let mut world = World::new();
        std::mem::swap(&mut self.world, &mut world);
        drop(world);
    }
}

impl<'a, 'b> GameBuilder<'a, 'b> {
    pub fn with_system<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where S: 'a + for<'d> System<'d> + Send + Sync {
        self.dispatcher.add(system, name, dependencies);
        self
    }
    pub fn with_event_system<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where S: 'a + for<'d> System<'d> + Send + Sync {
        self.event_dispatcher.add(system, name, dependencies);
        self
    }
    pub fn with_interpreter_system(mut self, system: InterpreterSystem) -> Self {
        self.interpreter_dispatcher.push(system);
        self
    }

    pub fn build(self) -> Game<'a, 'b> {
        Game {
            world: self.world,
            dispatcher: self.dispatcher.build(),
            event_dispatcher: self.event_dispatcher.build(),
            interpreter_dispatcher: self.interpreter_dispatcher,
            include_builtins: self.include_builtins
        }
    }
}

impl<'a, 'b> Default for GameBuilder<'a, 'b> {
    fn default() -> Self {
        GameBuilder {
            world: World::new(),
            dispatcher: DispatcherBuilder::new(),
            event_dispatcher: DispatcherBuilder::new(),
            interpreter_dispatcher: Vec::new(),
            include_builtins: true
        }
    }
}