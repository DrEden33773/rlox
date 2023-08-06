//! # VM
//!
//! ## Based on Stack
//!
//! The VM (aka. Virtual Machine) is the core of the interpreter.
//!
//! It is responsible for:
//!
//! - executing the bytecode

#[cfg(feature = "debug_trace_execution")]
use crate::debug::Debug;
use crate::{
  chunk::{Chunk, OpCode},
  utils::Init,
  value::Value,
};

/// ## InterpretError
///
/// An enum which represents the different errors that can occur
/// during the interpretation.
#[derive(Debug, Clone)]
pub enum InterpretError {
  CompileError(String),
  RuntimeError(String),
}

/// ## VM
///
/// A struct which represents the virtual machine.
#[derive(Debug, Default)]
pub struct VM {
  /// A reference to the chunk (or, None).
  pub(crate) chunk: Chunk,
  /// The instruction pointer (actually, the index).
  pub(crate) ip: usize,
  /// The stack of the virtual machine.
  pub(crate) stack: Vec<Value>,
}

impl VM {
  /// Interpret from string.
  pub fn interpret(&mut self, src: String) -> Result<(), InterpretError> {
    self.rebind(Chunk::init());
    self.compile(src)?;
    self.run()
  }

  /// Interpret from string, but only show tokens.
  pub fn interpret_to_token(&mut self, src: String) -> Result<(), InterpretError> {
    self.compile_to_token(src)
  }

  /// Interpret from file(path).
  pub fn interpret_file(&mut self, path: String) -> Result<(), InterpretError> {
    use std::fs::read_to_string;
    if let Ok(content) = read_to_string(path) {
      self.interpret_to_token(content)
    } else {
      Err(InterpretError::CompileError(
        "Failed to interpret from file.".into(),
      ))
    }
  }
}

impl VM {
  fn unary_op<T>(&mut self, op: T) -> Result<(), InterpretError>
  where
    T: Fn(Value) -> Result<Value, InterpretError>,
  {
    if let Some(value) = self.stack.pop() {
      self.stack.push(op(value)?);
      Ok(())
    } else {
      Err(InterpretError::RuntimeError(
        "Operate on an empty stack.".into(),
      ))
    }
  }

  fn binary_op<T>(&mut self, op: T) -> Result<(), InterpretError>
  where
    T: Fn(Value, Value) -> Result<Value, InterpretError>,
  {
    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
      self.stack.push(op(a, b)?);
      Ok(())
    } else {
      Err(InterpretError::RuntimeError(
        "Operate on an empty stack.".into(),
      ))
    }
  }
}

impl VM {
  /// Read a byte from the chunk (update ip).
  fn read_byte(&mut self) -> u8 {
    let byte = self.chunk.code[self.ip];
    self.ip += 1;
    byte
  }

  /// Read a constant from the chunk (update ip).
  fn read_constant(&mut self) -> Value {
    let index = self.chunk.code[self.ip];
    self.ip += 1;
    self.chunk.constants.values[index as usize]
  }
}

impl VM {
  /// Disassemble the current instruction.
  ///
  /// This function is only available when the feature
  /// `debug_trace_execution` is enabled.
  #[cfg(feature = "debug_trace_execution")]
  #[allow(dead_code)]
  fn disassemble_instruction(&self) -> Result<(), InterpretError> {
    self.chunk.disassemble_instruction(self.ip);
    Ok(())
  }

  /// Trace VM's stack.
  ///
  /// This function is only available when the feature
  /// `debug_trace_stack` is enabled.
  #[cfg(feature = "debug_trace_stack")]
  pub fn trace_stack(&self) {
    print!("        | ");
    print!("[");
    for (i, value) in self.stack.iter().enumerate() {
      print!("{}", value);
      if i != self.stack.len() - 1 {
        print!(", ");
      }
    }
    println!("]")
  }
}

impl VM {
  /// Link the given chunk to the virtual machine, then interpret it.
  pub fn interpret_chunk(&mut self, chunk: Chunk) -> Result<(), InterpretError> {
    println!("-x-x-x-x- Called : Chunk Interpreter -x-x-x-x-");
    self.chunk = chunk;
    self.ip = 0;
    if let Ok(()) = self.run() {
      println!("-x-x-x-x- End of : Chunk Interpreter -x-x-x-x-\n");
      return Ok(());
    }
    Err(InterpretError::RuntimeError(
      "Failed to run the chunk.".into(),
    ))
  }

  /// Run the virtual machine (with a valid chunk reference).
  pub fn run(&mut self) -> Result<(), InterpretError> {
    let mut result = Ok(());
    while self.ip < self.chunk.code.len() {
      #[cfg(feature = "debug_print_code")]
      {
        #[cfg(feature = "debug_trace_stack")]
        self.trace_stack();
        #[cfg(feature = "debug_trace_execution")]
        self.disassemble_instruction()?;
      }
      result = self.run_one_step();
      if result.is_err() {
        break;
      }
    }
    result
  }

  #[inline]
  fn run_one_step(&mut self) -> Result<(), InterpretError> {
    let raw_result = match self.read_byte().into() {
      OpCode::Constant => {
        let constant = self.read_constant();
        self.stack.push(constant);
        Ok(())
      }
      OpCode::Nil => {
        self.stack.push(Value::nil_val());
        Ok(())
      }
      OpCode::True => {
        self.stack.push(Value::bool_val(true));
        Ok(())
      }
      OpCode::False => {
        self.stack.push(Value::bool_val(false));
        Ok(())
      }
      OpCode::Equal => self.binary_op(|l, r| Ok(Value::bool_val(l == r))),
      OpCode::Greater => self.binary_op(|l, r| Ok(Value::bool_val(l > r))),
      OpCode::Less => self.binary_op(|l, r| Ok(Value::bool_val(l < r))),
      OpCode::NotEqual => self.binary_op(|l, r| Ok(Value::bool_val(l != r))),
      OpCode::GreaterEqual => self.binary_op(|l, r| Ok(Value::bool_val(l >= r))),
      OpCode::LessEqual => self.binary_op(|l, r| Ok(Value::bool_val(l <= r))),
      OpCode::Add => self.binary_op(|l, r| l + r),
      OpCode::Subtract => self.binary_op(|l, r| l - r),
      OpCode::Multiply => self.binary_op(|l, r| l * r),
      OpCode::Divide => self.binary_op(|l, r| l / r),
      OpCode::Not => self.unary_op(|v| !v),
      OpCode::Negate => self.unary_op(|v| -v),
      OpCode::Return => {
        if let Some(value) = self.stack.pop() {
          println!("=> {}", value);
        }
        return Ok(());
      }
    };
    if let Err(InterpretError::RuntimeError(message)) = raw_result {
      self.runtime_error(message)
    } else {
      Ok(())
    }
  }
}

impl VM {
  pub fn runtime_error(&mut self, message: String) -> Result<(), InterpretError> {
    // Index should be `ip - 1`, as ip has increased before error occurred.
    let inst_index = self.ip - 1;

    let line = self.chunk.lines[inst_index];
    let message = format!("[line {}] in script: {}", line, message);

    self.stack.clear();

    Err(InterpretError::RuntimeError(message))
  }
}

impl Init for VM {}

impl VM {
  /// Create a new virtual machine (with no chunk linked, ip as 0).
  pub fn init() -> Self {
    Self {
      chunk: Chunk::default(),
      ip: 0,
      stack: Vec::default(),
    }
  }

  /// Free the chunk (if any).
  pub fn free(&mut self) {
    // if let Some(ref mut chunk) = self.chunk {
    //   chunk.clear();
    // }
    self.stack.clear();
  }

  /// Rebind the virtual machine to the given chunk.
  pub fn rebind(&mut self, chunk: Chunk) {
    self.chunk = chunk;
    self.ip = 0;
  }
}
