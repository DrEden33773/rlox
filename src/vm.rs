//! # VM
//!
//! ## Based on Stack
//!
//! The VM (aka. Virtual Machine) is the core of the interpreter.
//!
//! It is responsible for:
//!
//! - executing the bytecode

use std::collections::VecDeque;

#[cfg(feature = "debug_trace_execution")]
use crate::debug::Debug;
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
  /// The stack of the virtual machine.
  pub(crate) stack: VecDeque<Value>,
}

impl<'a> VM<'a> {
  /// Interpret from string, with no chunk delegated.
  pub fn interpret_str(&mut self, src: &str) -> Result<(), InterpretError> {
    self.compile(src)
  }

  /// Interpret from file(path), with no chunk delegated.
  pub fn interpret_file(&mut self, path: &str) -> Result<(), InterpretError> {
    use std::fs::read_to_string;
    if let Ok(content) = read_to_string(path) {
      self.interpret_str(content.as_str())
    } else {
      Err(InterpretError::CompileError)
    }
  }
}

impl<'a> VM<'a> {
  fn monocular_op<T>(&mut self, op: T) -> bool
  where
    T: Fn(Value) -> Value,
  {
    if let Some(value) = self.stack.pop_back() {
      self.stack.push_back(op(value));
      true
    } else {
      false
    }
  }

  fn binary_op<T>(&mut self, op: T) -> bool
  where
    T: Fn(Value, Value) -> Value,
  {
    if let (Some(b), Some(a)) = (self.stack.pop_back(), self.stack.pop_back()) {
      self.stack.push_back(op(a, b));
      true
    } else {
      false
    }
  }
}

impl<'a> VM<'a> {
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
  /// Disassemble the current instruction.
  ///
  /// This function is only available when the feature
  /// `debug_trace_execution` is enabled.
  #[cfg(feature = "debug_trace_execution")]
  fn disassemble_instruction(&self) -> Result<(), InterpretError> {
    if let Some(ref chunk) = self.chunk {
      chunk.disassemble_instruction(self.ip);
      Ok(())
    } else {
      Err(InterpretError::RuntimeError)
    }
  }

  /// Trace VM's stack.
  ///
  /// This function is only available when the feature
  /// `debug_trace_stack` is enabled.
  #[cfg(feature = "debug_trace_stack")]
  pub fn trace_stack(&self) {
    print!("        | ");
    println!("{:?}", self.stack);
  }
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
    let mut result = Ok(());
    while self.ip < self.chunk.as_ref().unwrap().code.len() {
      #[cfg(feature = "debug_trace_stack")]
      self.trace_stack();
      #[cfg(feature = "debug_trace_execution")]
      self.disassemble_instruction()?;
      result = self.run_one_step();
      if result.is_err() {
        break;
      }
    }
    result
  }

  #[inline]
  fn run_one_step(&mut self) -> Result<(), InterpretError> {
    let no_crush_end = match self.read_byte()?.into() {
      OpCode::Constant => {
        let constant = self.read_constant()?;
        self.stack.push_back(constant);
        true
      }
      OpCode::Negate => self.monocular_op(|v| -v),
      OpCode::Return => {
        if let Some(value) = self.stack.pop_back() {
          println!("{}", value);
        }
        return Ok(());
      }
      OpCode::Add => self.binary_op(|l, r| l + r),
      OpCode::Subtract => self.binary_op(|l, r| l - r),
      OpCode::Multiply => self.binary_op(|l, r| l * r),
      OpCode::Divide => self.binary_op(|l, r| l / r),
    };
    if no_crush_end {
      Ok(())
    } else {
      Err(InterpretError::RuntimeError)
    }
  }
}

impl<'a> VM<'a> {
  /// Create a new virtual machine (with no chunk linked, ip as 0).
  pub fn init() -> Self {
    Self {
      chunk: None,
      ip: 0,
      stack: VecDeque::default(),
    }
  }

  /// Free the chunk (if any).
  pub fn free(&mut self) {
    if let Some(ref mut chunk) = self.chunk {
      chunk.clear();
    }
    self.stack.clear();
  }
}
