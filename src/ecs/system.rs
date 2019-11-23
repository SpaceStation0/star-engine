use super::scheduler::{ExecutionPolicy, ExecutionTime};
use std::marker::PhantomData;
use std::any::Any;
use super::accessor::SystemData;
use super::World;

pub trait System<'a>: Any {
    type SystemData: SystemData<'a>;

    fn execution_policy(&self) -> ExecutionPolicy;
    fn execution_time(&self) -> ExecutionTime;
    fn run(&mut self, data: Self::SystemData);
}

pub trait SystemInterface<'a> {
    fn execution_policy(&self) -> ExecutionPolicy;
    fn execution_time(&self) -> ExecutionTime;
    fn run_on_world(&mut self, world: &'a World);
}

impl<'a, T> SystemInterface<'a> for T
where T: System<'a> {
    fn execution_policy(&self) -> ExecutionPolicy {
        System::execution_policy(self)
    }
    fn execution_time(&self) -> ExecutionTime {
        System::execution_time(self)
    }
    fn run_on_world(&mut self, world: &'a World) {
        let data = T::SystemData::fetch(world);
        self.run(data);
    }
}


