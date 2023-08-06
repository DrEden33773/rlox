//! # Chunk
//!
//! A module which represents a sequence of bytecode,
//! with it's dependent components.

use crate::{
  utils::Init,
  value::{Value, ValueArray},
};
use enum_repr::EnumU8;

/// ## OpCode
///
/// An enum which represents the different opcodes used in the
/// virtual machine.
#[derive(EnumU8)]
pub enum OpCode {
  Constant,
  Nil,
  True,
  False,
  Equal,
  Greater,
  Less,
  NotEqual,
  GreaterEqual,
  LessEqual,
  Add,
  Subtract,
  Multiply,
  Divide,
  Not,
  Negate,
  Return,
}

/// ## Chunk
///
/// A struct which represents a chunk/sequence of bytecode.
#[derive(Debug, Default, Clone)]
pub struct Chunk {
  pub(crate) code: Vec<u8>,
  pub(crate) lines: Vec<usize>,
  pub(crate) constants: ValueArray,
}

impl Chunk {
  /// Write a byte to the given chunk.
  pub fn write_chunk(&mut self, byte: u8, line: usize) {
    self.code.push(byte);
    self.lines.push(line);
  }

  /// Add a constant to the given chunk,
  /// then return it's index.
  pub fn add_constant(&mut self, value: Value) -> usize {
    self.constants.write(value);
    self.constants.values.len() - 1
  }

  /// Clear the given chunk.
  pub fn clear(&mut self) {
    self.code.clear();
    self.constants.clear();
  }
}

impl Init for Chunk {}
