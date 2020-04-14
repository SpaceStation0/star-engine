use crate::network::*;
use specs::World;
use super::Updater;

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