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

  /// Print a constant instruction.
  fn constant_instruction(&self, name: &str, offset: usize) -> usize;

  /// Get the line number of the given offset.
  fn line_number(&self, offset: usize) -> usize;
}

impl Debug for Chunk {
  fn disassemble(&self, name: &str) {
    println!("\n-*-*-*-> Global Disassembler : {} <-*-*-*-", name);

    let mut offset = 0;
    while offset < self.code.len() {
      offset = self.disassemble_instruction(offset);
    }

    println!("-*-*-*-*-*-*-*- End of: {} -*-*-*-*-*-*-*-\n", name);
  }

  fn disassemble_instruction(&self, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && self.line_number(offset) == self.line_number(offset - 1) {
      print!("   | ");
    } else {
      print!("{:4} ", self.line_number(offset));
    }

    let instruction = self.code[offset];
    match instruction.try_into() {
      Ok(op_code) => match op_code {
        OpCode::Constant => self.constant_instruction("CONSTANT", offset),
        OpCode::Nil => self.simple_instruction("NIL", offset),
        OpCode::True => self.simple_instruction("TRUE", offset),
        OpCode::False => self.simple_instruction("FALSE", offset),
        OpCode::Add => self.simple_instruction("ADD", offset),
        OpCode::Subtract => self.simple_instruction("SUBTRACT", offset),
        OpCode::Multiply => self.simple_instruction("MULTIPLY", offset),
        OpCode::Divide => self.simple_instruction("DIVIDE", offset),
        OpCode::Not => self.simple_instruction("NOT", offset),
        OpCode::Negate => self.simple_instruction("NEGATE", offset),
        OpCode::Return => self.simple_instruction("RETURN", offset),
      },
      _ => {
        println!("Unknown opcode {}", instruction);
        offset + 1
      }
    }
  }

  fn simple_instruction(&self, name: &str, offset: usize) -> usize {
    println!("{}", name);
    // move 1 byte ahead
    offset + 1
  }

  fn constant_instruction(&self, name: &str, offset: usize) -> usize {
    let index = self.code[offset + 1];
    println!(
      "{:16} {:4} '{}'",
      name,
      index,
      self.constants.values[index as usize].as_number()
    );
    // move 2 byte ahead
    offset + 2
  }

  fn line_number(&self, offset: usize) -> usize {
    self.lines[offset]
  }
}
