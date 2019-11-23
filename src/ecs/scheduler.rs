use crate::ecs::system::{SystemInterface, System};
use specs::{Dispatcher, DispatcherBuilder};
use crate::ecs::world::World;

/// The `Scheduler` has a very important job. In essence,
/// it routes and controls the execution of all systems.
/// The default scheduler dispatches systems in parallel
/// using `rayon`, if possible.
pub trait Scheduler {
    fn dispatch(&mut self, world: &mut World, systems: &mut Vec<Box<dyn SystemInterface>>);
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionPolicy {
    EveryTick,
    EveryNTicks(u32),
    OnInitOnly,
    DontExecute,
    EventOnly
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionTime {
    VeryShort,
    Short,
    Average,
    Long,
    VeryLong
}

pub struct DefaultScheduler {
    tick_nonce: u128
}

impl Scheduler for DefaultScheduler {
    fn dispatch(&mut self, world: &mut World, systems: &mut Vec<Box<dyn SystemInterface>>) {
        self.tick_nonce += 1;
        for system in systems {
            use ExecutionPolicy::*;
            match system.execution_policy() {
                EveryTick => (),
                EveryNTicks(n) => {
                    if (n as u128) % self.tick_nonce != 0 {
                        continue
                    }
                },
                OnInitOnly => continue,
                EventOnly => continue,
                DontExecute => continue
            }
            system.run_on_world(&world);
        }
    }
}