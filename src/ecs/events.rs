use specs::prelude::{SystemData};
use specs::Resources;
use crate::ecs::notifier::{NotifierQueue};
use specs::shred::{ResourceId, Fetch};
use std::any::Any;
use crate::ecs::event::{Event, EventFilter};
use std::marker::PhantomData;
use std::slice::Iter;

pub struct Events<'a, E> {
    _phantom_data: PhantomData<E>,
    inner: Fetch<'a, NotifierQueue>,
}

pub struct EventsIterator<'a, E> {
    _phantom_data: PhantomData<E>,
    iter: Iter<'a, Box<dyn Event>>
}

impl<'a, E> Events<'a, E>
where E: EventFilter {
    pub fn iter(&self) -> EventsIterator<'a, E> {
        EventsIterator {
            _phantom_data: PhantomData,
            iter: unsafe {
                // This is a safe operation, because the event iterator's lifetime is
                // bounded to the associated Events
                (*self.queue_as_ptr()).iter()
            }
        }
    }
    fn queue_as_ptr(&self) -> *const NotifierQueue {
        &*self.inner as *const NotifierQueue
    }
}

impl<'a, E> SystemData<'a> for Events<'a, E>
where E: EventFilter {
    fn setup(res: &mut Resources) {
        res.insert(NotifierQueue::new());
    }

    fn fetch(res: &'a Resources) -> Events<'a, E> {
        let queue = res.fetch::<NotifierQueue>();
        Events {
            _phantom_data: PhantomData,
            inner: queue
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec!(ResourceId(NotifierQueue::new().type_id()))
    }
    fn writes() -> Vec<ResourceId> {
        vec!()
    }
}

impl<'a, E> EventsIterator<'a, E>
    where E: EventFilter {
    pub fn new(iter: Iter<'a, Box<dyn Event>>) -> EventsIterator<'a, E> {
        EventsIterator {
            _phantom_data: PhantomData,
            iter
        }
    }
}

impl<'a, E> Iterator for EventsIterator<'a, E>
    where E: EventFilter {
    type Item = &'a dyn Event;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(boxed) => {
                if E::has_type((**boxed).type_id()) {
                    Some(&**boxed)
                } else {
                    None
                }
            },
            None => None
        }
    }
}
