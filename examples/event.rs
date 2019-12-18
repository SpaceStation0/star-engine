#![feature(type_ascription)]
extern crate star_engine;
use star_engine::ecs::event::*;
use std::any::Any;
use specs::{System, Write, Resources};
use star_engine::ecs::notifier::{NotifierQueue};
use std::fs::File;
use star_engine::logger;
use star_engine::ecs::Game;
use star_engine::ecs::events::Events;
use cascade::cascade;
use specs::SystemData;

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

type DamageFilter = Attack;

fn main() {
    let file = File::create("./logs.txt").unwrap();
    logger::set_logging_output(file);
    let game = Game::new_builder();

    let mut res = Resources::new();
    let queue = cascade! {
        NotifierQueue::new();
        ..push_event(Attack { damage: 1, from: 0, to: 1});
        ..push_event(Attack { damage: 1, from: 1, to: 2});
    };
    res.insert(queue);
    let filter: Events<DamageFilter> = SystemData::fetch(&res);
    //let iter = filter.iter();
    //drop(filter);
    for i in filter.iter() {
        logger::info(format!("{:?}", (force_downcast_event_ref(i): &Attack)));
    }
}