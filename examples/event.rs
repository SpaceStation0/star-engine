extern crate star_engine;
use star_engine::ecs::event::*;
use std::any::Any;
use specs::{System, Write};
use star_engine::ecs::notifier::{NotifierQueue, NotifierCallback};

type PlayerID = u64;

struct Attack {
    pub damage: u32,
    pub from: PlayerID,
    pub to: PlayerID
}

impl Event for Attack {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

#[derive(Clone, Default)]
struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = Write<'a, NotifierQueue>;
    fn run(&mut self, mut notifier: Self::SystemData) {
        notifier.push_event(Attack { damage: 1, from: 0, to: 1});
    }
}

type DamageFilter = Attack;

impl<'a> NotifierCallback<'a> for DamageSystem {
    type CallbackData = ();
    fn callback(&mut self, event: Box<dyn Event>, _: Self::CallbackData) {
        if is::<DamageFilter>(&event) {
            let attack = force_downcast_event::<Attack>(event);
            println!("{}", attack.damage);
        }
    }
}

fn main() {

}