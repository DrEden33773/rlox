//! # VM
//!
//! The VM (aka. Virtual Machine) is the core of the interpreter.
//! It is responsible for executing the bytecode.

use crate::{
  chunk::{Chunk, OpCode},
  common::Value,
};

/// ## InterpretResult
///
/// An enum which represents the result of the interpretation.
#[derive(Debug, Clone)]
pub enum InterpretResult {
  Ok,
  CompileError,
  RuntimeError,
}

/// ## VM
///
/// A struct which represents the virtual machine.
#[derive(Debug)]
pub struct VM<'a> {
  pub(crate) chunk: Option<&'a mut Chunk>,
  pub(crate) ip: usize,
}

impl<'a> VM<'a> {
  pub fn interpret(&mut self, chunk: &'a mut Chunk) -> InterpretResult {
    self.chunk = Some(chunk);
    self.ip = 0;
    self.run()
  }

  pub fn run(&mut self) -> InterpretResult {
    while let Some(instruction) = self.read_byte() {
      match instruction.into() {
        OpCode::CONSTANT => {
          let constant = self.read_constant();
          println!("{}", constant.unwrap());
        }
        OpCode::RETURN => return InterpretResult::Ok,
      }
    }
    InterpretResult::Ok
  }

  fn read_byte(&mut self) -> Option<u8> {
    if let Some(ref chunk) = self.chunk {
      let byte = chunk.code[self.ip];
      self.ip += 1;
      return Some(byte);
    }
    None
  }

  fn read_constant(&mut self) -> Option<Value> {
    if let Some(ref chunk) = self.chunk {
      let index = chunk.code[self.ip];
      self.ip += 1;
      return Some(chunk.constants.values[index as usize]);
    }
    None
  }
}

impl<'a> VM<'a> {
  pub fn init() -> Self {
    Self { chunk: None, ip: 0 }
  }

  pub fn free(&mut self) {}
}
