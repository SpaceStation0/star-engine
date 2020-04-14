extern crate star_engine;

use star_engine::ecs::Game;
use star_engine::script::{PythonInterpreter, system::InterpreterSystem};
use specs::{World, System, Read};
use cascade::cascade;
use star_engine::logger::info;

fn run_interpreter(inter: &mut InterpreterSystem, world: &World) {}

fn test_interpreter() -> InterpreterSystem {
    let mut i = InterpreterSystem::new();
    i.include("./examples").unwrap();
    i.with_module("example").unwrap();
    i.with_read::<String>("something".to_string());
    return i;
}

struct ExampleSystem;

impl<'a> System<'a> for ExampleSystem {
    type SystemData = Read<'a, String>;

    fn run(&mut self, data: Self::SystemData) {
        info(format!("String resource is now equal to {}", *data));
    }
}

fn main() {
    let mut game = Game::new_builder().with_interpreter_system(test_interpreter()).build();
    game.tick();
}