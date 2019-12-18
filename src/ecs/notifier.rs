use super::event::*;
use std::slice::Iter;
/*
/// If you want a system to process something that would involve multiple systems,
/// you implement a notifier callback. Implementing NotifierCallback on a system
/// allows it to receive events sent by other systems (or itself).
///
/// In more technical terms, after the primary game tick, the event dispatch phase occurs.
/// During the event dispatch phase, events are popped off the NotifierQueue based on their
/// priority: a higher priority means they get called first. When an event gets popped, it
/// is sent to all systems that implement the NotifierCallback trait.
/// The Event will be sent as a Box<dyn Event>. Downcasting to a concrete event type
/// can be accomplished with `downcast_event` or `force_downcast_event`.
pub trait NotifierCallback<'a>: System<'a> {
    type EventFilter: EventFilter;

    fn callback(&mut self, event: Box<dyn Event>, data: Self::SystemData) {
        if !Self::EventFilter::has_type(event.type_id()) {
            return;
        }

    }
}
*/

pub struct NotifierQueue {
    queue: Vec<Box<dyn Event>>,
    needs_sort: bool
}

impl NotifierQueue {
    pub fn new() -> NotifierQueue {
        Default::default()
    }
    pub fn push_event<E: Event>(&mut self, event: E) {
        self.queue.push(Box::new(event));
        self.needs_sort = true;
    }
    pub fn pop_event(&mut self) -> Option<Box<dyn Event>> {
        if self.needs_sort {
            self.sort();
        }
        self.queue.pop()
    }
    pub fn iter(&self) -> Iter<'_, Box<dyn Event>> {
        self.queue.iter()
    }
    fn sort(&mut self) {
        self.queue.sort_by_key(|x| x.priority());
        self.needs_sort = false;
    }
}



impl Default for NotifierQueue {
    fn default() -> NotifierQueue {
        NotifierQueue { queue: vec!(), needs_sort: false }
    }
}