use crate::network::*;
use specs::World;

/// Updaters are payloads of data that can load themselves onto the world.
pub trait Updater {
    fn update_world(self, world: &mut World);
}

/// Handles messages from the client and makes the appropriate adjustments to the world
pub trait ClientMessageHandler {
    fn refresh_messages(&mut self, client_messages: ClientMessages, world: &mut World);
}

impl<T, U> ClientMessageHandler for T
where T: ClientMessageCodec<Output=U>, U: Updater {
    fn refresh_messages(&mut self, client_messages: ClientMessages, world: &mut World) {
        Updater::update_world(self.process_messages(client_messages), world);
    }
}