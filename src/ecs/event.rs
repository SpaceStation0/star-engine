use std::any::{Any, TypeId};
use std::fmt::Debug;

pub type EventID = TypeId;

pub trait EventFilter {
    fn has_type(event_id: EventID) -> bool;
}

pub trait Event : Any + Send + Sync + Debug {
    /// The priority of an event determines when it will be executed. The higher
    /// the priority, the sooner it will be called.
    fn priority(&self) -> u64 { 0 }

    fn as_any(&self) -> &dyn Any;
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

/// Get the `EventID` (which is just an alias for `std::any::TypeId`) of a certain event type.
pub fn id<E: Event>() -> EventID {
    EventID::of::<E>()
}

pub fn is<E: EventFilter>(event: &dyn Event) -> bool {
    E::has_type(event.type_id())
}

/// Downcast a dynamic box pointer to an `Event` trait object into a concrete type.
/// # Errors
/// If the given type doesn't match the actual type of the `Event` object,
/// this function will return the original object
pub fn downcast_event<E: Event>(mut event: Box<dyn Event>) -> Result<E, Box<dyn Event>> {
    // Check if the downcast will work so we can return the original object
    if !is::<E>(&*event) {
        return Err(event);
    }

    // Even though this has an unsafe block around it, the operation
    // is perfectly safe. What we're doing here is turing our argument into
    // a mutable pointer, and then making it owned by the box pointer.
    // Since the box pointer for the trait object has already been moved into this scope,
    // we aren't breaking borrowing rules; the reason why we need a pointer and not a reference
    // is because the borrow checker would complain that the reference outlives the scope.
    let event_any = unsafe {
        Box::from_raw(event.as_mut_any() as *mut dyn Any)
    };
    // The unwrap is safe because we already checked that the downcast would work
    Ok(*event_any.downcast().unwrap())
}

pub fn downcast_event_ref<E: Event>(event: &dyn Event) -> Result<&E, &dyn Event> {
    if !is::<E>(event) {
        return Err(event);
    }
    Ok(event.as_any().downcast_ref::<E>().unwrap())
}

/// Forcibly downcast a dynamic box pointer to an `Event` trait object into
/// a concrete type.
/// # Panics
/// This will panic if the cast fails. Only call this if you're certain about the
/// type of the event!
pub fn force_downcast_event<E: Event>(event: Box<dyn Event>) -> E {
    downcast_event(event).expect("Downcast of event failed")
}

pub fn force_downcast_event_ref<E: Event>(event: &dyn Event) -> &E {
    downcast_event_ref(event).expect("Downcast of event reference failed")
}

impl<A> EventFilter for A
where A: Event {
    fn has_type(event_id: EventID) -> bool {
        id::<A>() == event_id
    }
}

impl<A, B> EventFilter for (A, B)
    where A: Event, B: Event {
    fn has_type(event_id: EventID) -> bool {
        id::<A>() == event_id || id::<B>() == event_id
    }
}

impl<A, B, C> EventFilter for (A, B, C)
    where A: Event, B: Event, C: Event {
    fn has_type(event_id: EventID) -> bool {
        id::<A>() == event_id || id::<B>() == event_id
        || id::<C>() == event_id
    }
}

impl<A, B, C, D> EventFilter for (A, B, C, D)
    where A: Event, B: Event, C: Event, D: Event {
    fn has_type(event_id: EventID) -> bool {
        id::<A>() == event_id || id::<B>() == event_id
        || id::<C>() == event_id || id::<D>() == event_id
    }
}

impl<A, B, C, D, E> EventFilter for (A, B, C, D, E)
    where A: Event, B: Event, C: Event, D: Event, E: Event {
    fn has_type(event_id: EventID) -> bool {
        id::<A>() == event_id || id::<B>() == event_id
        || id::<C>() == event_id || id::<D>() == event_id
        || id::<E>() == event_id
    }
}