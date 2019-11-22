use crate::ecs::event::*;
use std::any::Any;
use std::mem::swap;

/// Manages the execution of event-based notifiers,
///
pub struct EventManager {
    notifiers: Vec<Box<dyn Notifier>>,
    event_queue: Events
}

pub trait Notifier {
    fn filter(&self) -> Vec<EventID>;
    fn notify(&mut self, e: Box<dyn Event>) -> Events;
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

    pub fn run_cycle(&mut self) {
        let mut working_queue = Events::new();
        swap(&mut self.event_queue,&mut working_queue);
        for mut event in working_queue {
            for mut notifier in &mut self.notifiers {
                let filter = notifier.filter();
                if filter.contains(&event.id()) {
                    let events = notifier.notify(event);
                    self.event_queue += events;
                    break;
                }
            }
        }
    }

    pub fn event_count(&self) -> usize {
        self.event_queue.inner().len()
    }
}