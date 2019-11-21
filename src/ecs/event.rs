pub use std::any::{TypeId, Any};
use std::ops::{Index, Add, AddAssign};
use std::cmp::Ordering;

pub type EventID = TypeId;

pub fn id<E: Event>() -> EventID {
    EventID::of::<E>()
}

pub trait Event : Any {
    fn priority(&self) -> u64 { 0 }
    fn id(&self) -> EventID {
        self.type_id()
    }
}

impl PartialOrd<dyn Event> for dyn Event {
    fn partial_cmp(&self, rhs: &dyn Event) -> Option<Ordering> {
        self.priority().partial_cmp(&rhs.priority())
    }
}

impl PartialEq<dyn Event> for dyn Event {
    fn eq(&self, rhs: &dyn Event) -> bool {
        self.id() == rhs.id()
    }
}

impl Eq for dyn Event {}


impl Ord for dyn Event {
    fn cmp(&self, other: &dyn Event) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

pub fn is<E:Event + Sized>(any: &Box<dyn Any>) -> bool {
    any.is::<E>()
}

pub fn downcast<E:Event + Sized>(event: Box<dyn Any>) -> Result<E, Box<dyn Any>> {
    match event.downcast() {
        Ok(b) => Ok(*b),
        Err(e) => Err(e)
    }
}

pub fn force_downcast<E:Event + Sized>(event: Box<dyn Any>) -> E {
    downcast::<E>(event).expect("Downcasted to wrong event type")
}

// Note: the bool value is for checking whether the event list
// needs to be shuffled, so that events can be re-arranged lazily.
pub struct Events(Vec<Box<dyn Event>>, bool);

impl Events {
    pub fn new() -> Events {
        Events(Vec::new(), true)
    }
    pub fn push<E>(&mut self, e: E) where E: Event {
        self.0.push(Box::new(e));
        self.1 = false;
    }
    pub fn inner(&mut self) -> &mut Vec<Box<dyn Event>> {
        &mut self.0
    }
    fn sort(&mut self) {
        self.0.sort_unstable();
    }
}

impl Add for Events {
    type Output = Events;

    fn add(mut self, mut rhs: Events) -> Events {
        self.0.append(&mut rhs.0);
        self.sort();
        self
    }
}

impl AddAssign for Events {

    fn add_assign(&mut self, mut rhs: Events) {
        self.0.append(&mut rhs.0);
        self.sort();
    }
}

impl<'a> IntoIterator for Events {
    type Item = Box<dyn Event>;
    type IntoIter = <Vec<Box<dyn Event>> as IntoIterator>::IntoIter;
    fn into_iter(mut self) -> Self::IntoIter {
        // Sort elements before iteration
        if self.1 {
            self.sort();
            self.1 = false;
        }
        self.0.into_iter()
    }
}