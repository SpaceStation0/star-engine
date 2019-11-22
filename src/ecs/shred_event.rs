use specs::*;
use std::any::{Any, TypeId};

pub type EventID = TypeId;

/// Get the EventID (which is just an alias for `std::any::TypeId`) of a certain event type.
pub fn id<E: Event>() -> EventID {
    EventID::of::<E>()
}

/// Downcast a dynamic box pointer to an `Event` trait object into a concrete type.
/// This may fail, and will return a box pointer to the `dyn Any` version of this object.
pub fn downcast_event<E: Event>(mut event: Box<dyn Event>) -> Result<E, Box<dyn Any>> {
    // Even though this has an unsafe block around it, the operation
    // is perfectly safe. What we're doing here is turing our argument into
    // a mutable pointer, and then making it owned by the box pointer.
    // Since the box pointer for the trait object has already been moved into this scope,
    // we aren't breaking borrowing rules; the reason why we need a pointer and not a reference
    // is because the borrow checker would complain that the reference outlives the scope.
    let event_any = unsafe {
        Box::from_raw(event.as_mut_any() as *mut dyn Any)
    };
    match event_any.downcast() {
        Ok(e) => Ok(*e),
        Err(box_any) => Err(box_any)
    }
}

/// Forcibly downcast a dynamic box pointer to an `Event` trait object into
/// a concrete type.
/// # Panics
/// This will panic if the cast fails. Only call this if you're certain about the
/// type of the event!
pub fn force_downcast_event<E: Event>(mut event: Box<dyn Event>) -> E {
    downcast_event(event).expect("Downcast of event failed")
}


pub trait Event : Any {
    /// The priority of an event determines when it will be executed. The higher
    /// the priority, the sooner it will be called.
    fn priority(&self) -> u64 { 0 }
    /// To properly downcast to a concrete type, an Event
    /// must provide a mutable reference to it's representation as
    /// an `Any` object. Due to restrictions of Rust, this must be done
    /// manually for each trait.
    /// Unless there's a *very* good reason for it, you should implement
    /// this function like so:
    /// ```
    /// impl Event for SomeStruct {
    ///     fn as_mut_any(&mut self) -> &mut dyn Any {
    ///         self as &mut dyn Any
    ///     }
    /// }
    /// ```
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

/// If you want a system to process something that would involve multiple systems,
/// you implement a notifier callback. Implementing NotifierCallback on a system
/// allows it to recieve events sent by other systems (or itself).
///
/// In more technical terms, after the primary game tick, the event dispatch phase occurs.
/// During the event dispatch phase, events are popped off the NotifierQueue based on their
/// priority: a higher priority means they get called first. When an event gets popped, it
/// is sent to all systems that implement the NotifierCallback trait.
/// The Event will be sent as a Box<dyn Event>. Downcasting to a concrete event type
/// can be accomplished with `downcast_event` or `force_downcast_event`.
pub trait NotifierCallback<'a>: System<'a> {

}