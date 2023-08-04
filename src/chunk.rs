//! # Chunk
//!
//! A module which represents a sequence of bytecode,
//! with it's dependent components.

use crate::{
  common::{Value, ValueArray},
  utils::Init,
};
use enum_repr::EnumFromU8;

/// ## OpCode
///
/// An enum which represents the different opcodes used in the
/// virtual machine.
#[repr(u8)]
#[derive(EnumFromU8)]
pub enum OpCode {
  RETURN = 0,
}

/// ## Chunk
///
/// A struct which represents a chunk/sequence of bytecode.
#[derive(Debug, Default, Clone)]
pub struct Chunk {
  pub(crate) code: Vec<u8>,
  pub(crate) constants: ValueArray,
}

impl Chunk {
  /// Write a byte to the given chunk.
  pub fn write(&mut self, byte: u8) {
    self.code.push(byte);
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
