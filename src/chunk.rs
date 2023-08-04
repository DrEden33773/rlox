//! # Chunk
//!
//! A module which represents a sequence of bytecode,
//! with it's dependent components.

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
}

impl Chunk {
  /// Creates a new [`Chunk`].
  pub fn new(code: Vec<u8>) -> Self {
    Self { code }
  }

  /// Creates a new [`Chunk`] with a given capacity.
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      code: Vec::with_capacity(capacity),
    }
  }
}

impl Chunk {
  /// Write a byte to the given chunk.
  pub fn write(&mut self, byte: u8) {
    self.code.push(byte);
  }

  /// Clear the given chunk.
  pub fn clear(&mut self) {
    self.code.clear();
  }
}
