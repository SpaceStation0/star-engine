use star_engine::network::*;

fn main() {
    Server::new(BlankCodec).start();
}