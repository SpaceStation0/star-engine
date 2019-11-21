use crate::ecs::event::*;
use std::any::Any;

pub struct EventManager {
    notifiers: Vec<Box<dyn Any>>,
    event_queue: Events
}

pub trait Notifier: Sized {
    fn filter(&self) -> Vec<EventID>;
    fn notify(&mut self, e: Box<dyn Any>) -> Events;
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            notifiers: vec!(),
            event_queue: Events::new()
        }
    }

    pub fn add_notifier<N: Notifier + 'static>(&mut self, notifier: N) {
        self.notifiers.push(Box::new(notifier));
    }

    pub fn append_event_queue(&mut self, events: Events) {
        self.event_queue += events;
    }
}