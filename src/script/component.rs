use cpython::{ToPyObject, FromPyObject};
use specs::Component;

/// Sometimes, a Rust-based component needs to interfere with a
/// Python-based component. This can be done using `ScriptComponent`.
/// `ScriptComponent` is a special supertrait over the `specs::Component`
/// trait that require `FromPyObject` and `ToPyObject` to be implemented
/// for it. Access can be accomplished through `ScriptRead` and `ScriptWrite`,
/// special accessors that will read from the Python interpreter and
/// convert the object into the correct object.
pub trait ScriptComponent<'a>: Component + FromPyObject<'a> + ToPyObject {
    fn comp_name() -> &'static str;
}