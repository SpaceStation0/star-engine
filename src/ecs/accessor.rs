use super::World;
use std::any::{TypeId, Any};
use std::marker::PhantomData;
use specs::shred::{FetchMut, Fetch};

pub trait SystemData<'a> {
    fn fetch(world: &'a World) -> Self;

    fn reads() -> Vec<TypeId>;

    fn writes() -> Vec<TypeId>;
}

pub struct Read<'a, T> {
    _pd: PhantomData<&'a ()>,
    resource: Fetch<'a, T>
}

pub struct Write<'a, T> {
    _pd: PhantomData<&'a T>,
    resource: FetchMut<'a, T>
}

impl<'a, T> SystemData<'a> for Read<'a, T> {
    fn fetch(world: &'a World) -> Self {
        Read {
            _pd: PhantomData,
            resource: world.fetch::<T>()
        }
    }
    fn reads() -> Vec<TypeId> {
        vec!(T::type_id())
    }
    fn writes() -> Vec<TypeId> {
        vec!(T::type_id())
    }
}