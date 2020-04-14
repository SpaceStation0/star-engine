use super::*;
use specs::{World};
use shred::{Resource, ResourceId};
use cpython::ToPyObject;
use crate::logger::info;

enum Accessor {
    Read(String),
    Write(String),
    Entities
}


struct PythonSystem {
    name: String,
    accessors: Vec<Accessor>
}


/// The InterpreterSystem executes python modules. It is not
/// a regular system, because it requires access to the entire
/// world and will read/write from it dynamically.
pub struct InterpreterSystem {
    pub read_resource_map: HashMap<String, ResourceId>,
    pub write_resource_map: HashMap<String, ResourceId>,
    interpreter: PythonInterpreter,
    modules: HashMap<String, u64>,
    systems: HashMap<u64, PythonSystem>
}

impl PythonSystem {
    pub fn new(name: String, accessors: Vec<Accessor>) -> PythonSystem {
        PythonSystem {
            name,
            accessors
        }
    }
}

impl InterpreterSystem {
    pub fn new() -> InterpreterSystem {
        InterpreterSystem {
            read_resource_map: HashMap::new(),
            write_resource_map: HashMap::new(),
            interpreter: PythonInterpreter::new(),
            modules: HashMap::new(),
            systems: HashMap::new()
        }
    }

    pub fn include(&mut self, path: &'static str) -> InterpreterResult<()> {
        self.interpreter.include(path)?;
        Ok(())
    }

    pub fn with_read<T: Resource + ToPyObject>(&mut self, name: String) {
        self.read_resource_map.insert(name, ResourceId::new::<T>());
    }

    pub fn with_write<T: Resource + ToPyObject>(&mut self, name: String) {
        self.write_resource_map.insert(name, ResourceId::new::<T>());
    }

    pub fn with_module(&mut self, module: &str) -> InterpreterResult<()> {
        let id = self.interpreter.load_module(module)?;
        self.modules.insert(String::from(module), id);
        Ok(())
    }

    pub fn run(&mut self, world: &World) {
        for (module, id) in self.modules.iter() {
            // Get module systems
            self.get_module_systems(*id);
        }
    }

    fn get_module_systems(&self, id: u64) -> InterpreterResult<Vec<String>> {
        match self.interpreter.get_value(id, "systems") {
            Ok(py_obj) => {
                Ok(vec!())
            },
            Err(_) => {
                info(format!("The script with id ({}) has no 'systems' variable, and therefore will not be processed.", id));
                Ok(vec!())
            }
        }
    }
}