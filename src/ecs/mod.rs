//! Star Engine is powered by a robust ECS system. This system
//! executes in multiple phases and is designed to be fast,
//! easily extendable, and endlessly configurable.
//! Everything that happens inside the game is controlled from here.
//!
//! Star Engine has a tick-based design. Each tick, a scheduler
//! designates what systems should be executed. By default, this
//! scheduler will read the execution policies of systems and
//! execute them however often they need to be executed.
//! But the scheduler can be configured and adjusted to better
//! suit the needs of a certain application.
//!
//! Everything that happens occurs in the `Game` class. This
//! is the container for all sub-elements of the game. These
//! sub-elements include:
//! - The entity world
//! - The scheduler
//! - The system dispatcher
//! - The resource allocator
//! - The client handler
//! - The high-level game controllers
//! - The scripting engine
//!
//! Usually, the main function of an actual game
//! will be initializing everything in a `Game` struct
//! and then running it.
//!
//! Each sub-element of the `Game` struct is documented in
//! each of their respective folders.

pub mod world;
pub mod system;
pub mod event;
pub mod notifier;
pub mod scheduler;
pub mod accessor;
use world::World;
use scheduler::Scheduler;

/// The main struct, from which all game execution
/// occurs. A container of all engine elements.
pub struct Game {
    entity_world: World,

}

