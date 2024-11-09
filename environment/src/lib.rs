mod error;
mod function;

pub use error::EnvironmentError;
pub use function::*;

use types::Struct;

/// Environment is used to store all the registered functions and structures
/// It is used to give a context/std library to the parser / interpreter / VM
pub struct Environment {
    functions: Vec<NativeFunction>,
    structures: Vec<Struct>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            functions: Vec::new(),
            structures: Vec::new(),
        }
    }
}

impl Environment {
    // Create a new environment
    pub fn new() -> Self {
        Self::default()
    }

    // Get all the registered functions
    #[inline(always)]
    pub fn get_functions(&self) -> &Vec<NativeFunction> {
        &self.functions
    }

    // Get all the registered structures
    #[inline(always)]
    pub fn get_structures(&self) -> &Vec<Struct> {
        &self.structures
    }

    // Add a new function to the environment
    #[inline(always)]
    pub fn add_function(&mut self, function: NativeFunction) {
        self.functions.push(function);
    }

    // Add a new structure to the environment
    #[inline(always)]
    pub fn add_structure(&mut self, structure: Struct) {
        self.structures.push(structure);
    }
}