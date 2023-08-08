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

  /// Print a byte instruction (mainly used for local_variables).
  fn byte_instruction(&self, name: &str, offset: usize) -> usize;

  /// Print a full bunch of jump instruction
  fn jump_instruction(&self, name: &str, sign: usize, offset: usize) -> usize;
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
        OpCode::Constant => self.constant_instruction("</Constant/>", offset),
        OpCode::Nil => self.simple_instruction("</Nil/>", offset),
        OpCode::True => self.simple_instruction("</True/>", offset),
        OpCode::False => self.simple_instruction("</False/>", offset),
        OpCode::Equal => self.simple_instruction("@ Equal", offset),
        OpCode::Greater => self.simple_instruction("@ Greater", offset),
        OpCode::Less => self.simple_instruction("@ Less", offset),
        OpCode::NotEqual => self.simple_instruction("@ NotEqual", offset),
        OpCode::GreaterEqual => self.simple_instruction("@ GreaterEqual", offset),
        OpCode::LessEqual => self.simple_instruction("@ LessEqual", offset),
        OpCode::Add => self.simple_instruction("@ Add", offset),
        OpCode::Subtract => self.simple_instruction("@ Subtract", offset),
        OpCode::Multiply => self.simple_instruction("@ Multiply", offset),
        OpCode::Divide => self.simple_instruction("@ Divide", offset),
        OpCode::Not => self.simple_instruction("@ Not", offset),
        OpCode::Negate => self.simple_instruction("@ Negate", offset),
        OpCode::JumpIfFalse => self.jump_instruction("=>JumpIfFalse", 1, offset),
        OpCode::Jump => self.jump_instruction("=>Jump", 1, offset),
        OpCode::Print => self.simple_instruction("..Print", offset),
        OpCode::Pop => self.simple_instruction("..Pop", offset),
        OpCode::DefineGlobal => self.constant_instruction(":=DefineGlobal", offset),
        OpCode::GetGlobal => self.constant_instruction("<-GetGlobal", offset),
        OpCode::GetLocal => self.byte_instruction("<-GetLocal", offset),
        OpCode::SetGlobal => self.constant_instruction("->SetGlobal", offset),
        OpCode::SetLocal => self.byte_instruction("->SetLocal", offset),
        OpCode::Return => self.simple_instruction("..Return", offset),
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
      "{:16} {:4} :: {}",
      name, index, self.constants.values[index as usize]
    );
    // move 2 byte ahead
    offset + 2
  }

  fn byte_instruction(&self, name: &str, offset: usize) -> usize {
    let slot = self.code[offset + 1];
    println!("{:16} {:4}(slot)", name, slot);
    // move 2 byte ahead
    offset + 2
  }

  fn jump_instruction(&self, name: &str, sign: usize, offset: usize) -> usize {
    let jump = ((self.code[offset + 1] as u16) << 8) | self.code[offset + 2] as u16;
    println!(
      "{:16} {:4} -> {}",
      name,
      offset,
      offset + 3 + sign * jump as usize
    );
    offset + 3
  }

  fn line_number(&self, offset: usize) -> usize {
    self.lines[offset]
  }
}
