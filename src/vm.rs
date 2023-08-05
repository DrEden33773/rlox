//! # VM
//!
//! The VM (aka. Virtual Machine) is the core of the interpreter.
//! It is responsible for executing the bytecode.

use crate::{
  chunk::{Chunk, OpCode},
  common::Value,
};

/// ## InterpretError
///
/// An enum which represents the different errors that can occur
/// during the interpretation.
#[derive(Debug, Clone)]
pub enum InterpretError {
  CompileError,
  RuntimeError,
}

/// ## VM
///
/// A struct which represents the virtual machine.
#[derive(Debug)]
pub struct VM<'a> {
  /// A reference to the chunk (or, None).
  pub(crate) chunk: Option<&'a mut Chunk>,
  /// The instruction pointer (actually, the index).
  pub(crate) ip: usize,
}

impl<'a> VM<'a> {
  /// Link the given chunk to the virtual machine, then interpret it.
  pub fn interpret(&mut self, chunk: &'a mut Chunk) -> Result<(), InterpretError> {
    println!("-x-x-x-x- Called : Interpreter -x-x-x-x-");
    self.chunk = Some(chunk);
    self.ip = 0;
    if let Ok(()) = self.run() {
      println!("-x-x-x-x- End of : Interpreter -x-x-x-x-\n");
      return Ok(());
    }
    Err(InterpretError::RuntimeError)
  }

  /// Run the virtual machine (with a valid chunk reference).
  pub fn run(&mut self) -> Result<(), InterpretError> {
    while let Ok(instruction) = self.read_byte() {
      let no_crush_end = match instruction.into() {
        OpCode::CONSTANT => {
          let constant = self.read_constant()?;
          println!("{}", constant);
          true
        }
        OpCode::RETURN => return Ok(()),
      };
      if no_crush_end {
        return Ok(());
      }
    }
    Err(InterpretError::RuntimeError)
  }

  /// Read a byte from the chunk (update ip).
  fn read_byte(&mut self) -> Result<u8, InterpretError> {
    if let Some(ref chunk) = self.chunk {
      let byte = chunk.code[self.ip];
      self.ip += 1;
      return Ok(byte);
    }
    Err(InterpretError::RuntimeError)
  }

  /// Read a constant from the chunk (update ip).
  fn read_constant(&mut self) -> Result<Value, InterpretError> {
    if let Some(ref chunk) = self.chunk {
      let index = chunk.code[self.ip];
      self.ip += 1;
      return Ok(chunk.constants.values[index as usize]);
    }
    Err(InterpretError::RuntimeError)
  }
}

impl<'a> VM<'a> {
  /// Create a new virtual machine (with no chunk linked, ip as 0).
  pub fn init() -> Self {
    Self { chunk: None, ip: 0 }
  }

  /// Free the chunk (if any).
  pub fn free(&mut self) {
    if let Some(ref mut chunk) = self.chunk {
      chunk.clear();
    }
  }
}
