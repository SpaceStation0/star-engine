#![feature(type_ascription)]
extern crate star_engine;
use star_engine::ecs::event::*;
use std::any::Any;
use specs::{System, Write};
use star_engine::ecs::notifier::{NotifierQueue, NotifierCallback};
use std::fs::File;
use star_engine::logger;
use star_engine::ecs::Game;

type PlayerID = u64;

#[derive(Debug)]
struct Attack {
    pub damage: u32,
    pub from: PlayerID,
    pub to: PlayerID
}

impl Event for Attack {
    fn as_any(&self) -> &dyn Any { self as &dyn Any }
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

impl<'a> NotifierCallback<'a> for DamageSystem {
    type Filter = DamageFilter;
    fn handle_event(event: &dyn Event) {
        let attack: &Attack = force_downcast_event_ref(event);
        logger::info(format!("{:?}", attack));
    }
}

type DamageFilter = Attack;

fn main() -> Result<(), ()> {
    let file = File::create("./logs.txt").unwrap();
    logger::set_logging_output(file);
    let mut game = Game::new_builder().with_system(DamageSystem {}, "damage", &[]).build();
    game.tick()?;
    Ok(())
}