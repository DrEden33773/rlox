use super::*;

impl Parser {
  /// Report error at current token.
  pub(crate) fn error_at_current(&mut self, message: String) -> Result<(), InterpretError> {
    self.error_at(true, message)
  }

  // Report error at previous token.
  pub(crate) fn error(&mut self, message: String) -> Result<(), InterpretError> {
    self.error_at(false, message)
  }

  /// Report error at selected token.
  pub(crate) fn error_at(
    &mut self,
    if_current: bool,
    message: String,
  ) -> Result<(), InterpretError> {
    if self.panic_mode {
      return Ok(());
    }
    self.panic_mode = true;
    let token = if if_current {
      &self.current
    } else {
      &self.previous
    };
    let mut error_str = String::new();
    error_str += &format!("[line {}] Error", token.line);
    match token.token_type {
      TokenType::Eof => error_str += " at end",
      TokenType::Error => {}
      _ => error_str += &format!(" at '{}'", token.lexeme),
    }
    error_str += &format!(": {}", message);
    self.had_error = true;
    Err(InterpretError::CompileError(error_str))
  }
}
