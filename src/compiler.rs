//! # Compiler
//!
//! The compiler is responsible for compiling source_code into bytecode. It is
//! responsible for executing the bytecode.

use crate::vm::{InterpretError, VM};

impl<'a> VM<'a> {
  pub(crate) fn compile(&mut self, _src: &str) -> Result<(), InterpretError> {
    Ok(())
  }
}
