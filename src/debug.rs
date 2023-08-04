//! # Debug
//!
//! A module which represents the debugging utilities for the virtual machine.

use crate::chunk::{Chunk, OpCode};

pub trait Debug {
  /// Disassemble the given chunk.
  fn disassemble(&self, name: &str);

  /// Disassemble the given instruction.
  fn disassemble_instruction(&self, offset: usize) -> usize;

  /// Print a simple instruction.
  fn simple_instruction(&self, name: &str, offset: usize) -> usize;

  /// Get the line number of the given offset.
  fn line_number(&self, offset: usize) -> usize;
}

impl Debug for Chunk {
  /// Disassemble the given chunk.
  fn disassemble(&self, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < self.code.len() {
      offset = self.disassemble_instruction(offset);
    }
  }

  /// Disassemble the given instruction.
  fn disassemble_instruction(&self, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && self.line_number(offset) == self.line_number(offset - 1) {
      print!("   | ");
    } else {
      print!("{:4} ", self.line_number(offset));
    }

    let instruction = self.code[offset];
    match OpCode::from(instruction) {
      OpCode::RETURN => self.simple_instruction("RETURN", offset),
    }
  }

  /// Print a simple instruction.
  fn simple_instruction(&self, name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
  }

  /// Get the line number of the given offset.
  fn line_number(&self, _offset: usize) -> usize {
    0
  }
}
