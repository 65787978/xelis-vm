mod manager;
mod reader;

pub use manager::ChunkManager;
pub use reader::ChunkReader;

use crate::Value;

use super::OpCode;

// Each chunk is a collection of opcodes and constants
// It represent a function or a block of code
#[derive(Debug)]
pub struct Chunk {
    // All the constants used in the chunk
    constants: Vec<Value>,
    // All the opcodes defined in the chunk
    instructions: Vec<u8>
}

impl Chunk {
    // Create a new chunk
    #[inline]
    pub fn new() -> Self {
        Chunk {
            constants: Vec::new(),
            instructions: Vec::new()
        }
    }

    // Get the constant at the given index
    #[inline]
    pub fn get_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }

    // Get the opcodes length
    #[inline]
    pub fn index(&self) -> usize {
        self.instructions.len()
    }

    // Add a constant and retrieve its index
    #[inline]
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    // Pop the latest instruction
    pub fn pop_instruction(&mut self) {
        self.instructions.pop();
    }

    // Emit an opcode
    #[inline]
    pub fn emit_opcode(&mut self, op_code: OpCode) {
        self.instructions.push(op_code.as_byte());
    }

    #[inline]
    pub fn write_u8(&mut self, byte: u8) {
        self.instructions.push(byte);
    }

    #[inline]
    pub fn write_bool(&mut self, byte: bool) {
        self.instructions.push(byte as u8);
    }

    #[inline]
    pub fn write_u16(&mut self, byte: u16) {
        let bytes = byte.to_be_bytes();
        self.instructions.extend_from_slice(&bytes);
    }

    #[inline]
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.instructions.extend_from_slice(bytes);
    }
}
