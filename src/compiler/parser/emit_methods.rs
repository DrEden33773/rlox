use super::*;

impl Parser {
  /// Emit(pre) jump instructions, and tells `ip`
  /// which instruction to jump to **iff** the condition is false.
  ///
  /// Conclusion is that, just need to jump to `self.chunk.code.len() - 2`.
  ///
  /// (Jump back, only need to jump ahead over `if` and `condition` statement)
  pub(crate) fn emit_jump(&mut self, instruction: u8) -> Result<usize, InterpretError> {
    self.emit_byte(instruction)?;
    self.emit_bytes(&[0xff, 0xff])?;
    // -2 => `if` + `condition` = 2_bytes
    Ok(self.chunk.code.len() - 2)
  }

  /// Patch the jump instruction correctly.
  pub(crate) fn patch_jump(&mut self, offset: usize) -> Result<(), InterpretError> {
    // -2 to adjust for the bytecode for the jump offset itself
    let jump = self.chunk.code.len() - offset - 2;

    if jump > u16::MAX as usize {
      return Err(InterpretError::CompileError(
        "Too much code to jump over.".into(),
      ));
    }

    self.chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
    self.chunk.code[offset + 1] = (jump & 0xff) as u8;

    Ok(())
  }

  /// Appending a sequence of bytes to the chunk (in order).
  pub(crate) fn emit_bytes(&mut self, bytes: &[u8]) -> Result<(), InterpretError> {
    for &byte in bytes {
      self.emit_byte(byte)?;
    }
    Ok(())
  }

  /// Appending a single byte to the chunk.
  pub(crate) fn emit_byte(&mut self, byte: u8) -> Result<(), InterpretError> {
    self.chunk.write_chunk(byte, self.previous.line);
    Ok(())
  }

  /// Specifically appending the return instruction to the chunk.
  pub(crate) fn emit_return(&mut self) -> Result<(), InterpretError> {
    self.emit_byte(OpCode::Return as u8)
  }

  /// Wrapper for appending `constant` and `index` info to the chunk.
  pub(crate) fn emit_constant(&mut self, value: Value) -> Result<(), InterpretError> {
    let constant_index = self.make_constant(value)?;
    self.emit_bytes(&[OpCode::Constant as u8, constant_index])
  }

  /// Operations after end of compilation.
  pub(crate) fn end_compiler(&mut self) -> Result<(), InterpretError> {
    self.emit_return()
  }
}
