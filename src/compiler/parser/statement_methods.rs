use super::*;

impl Parser {
  pub(crate) fn print_statement(&mut self) -> Result<(), InterpretError> {
    self.expression()?;
    self.consume_token(TokenType::Semicolon, "Expect `;` after value.".into())?;
    self.emit_byte(OpCode::Print as u8)
  }

  pub(crate) fn if_statement(&mut self) -> Result<(), InterpretError> {
    self.consume_token(TokenType::LeftParen, "Expect `(` after `if`.".into())?;
    self.expression()?;
    self.consume_token(TokenType::RightParen, "Expect `)` after condition.".into())?;

    let then_jump = self.emit_jump(OpCode::JumpIfFalse as u8)?;
    self.statement()?;

    self.patch_jump(then_jump)
  }

  /// If in panic_mode, then synchronize (for better recognizing what error has occurred).
  ///
  /// Synchronize means that, we will skip tokens indiscriminately
  /// until we reach something that looks like a statement boundary.
  ///
  /// E.g.: class | fun | var | for | if | while | print | return
  pub(crate) fn synchronize(&mut self) -> Result<(), InterpretError> {
    self.panic_mode = false;
    while self.current.token_type != TokenType::Eof {
      if self.previous.token_type == TokenType::Semicolon {
        return Ok(());
      }
      match self.current.token_type {
        TokenType::Class
        | TokenType::Fun
        | TokenType::Var
        | TokenType::For
        | TokenType::If
        | TokenType::While
        | TokenType::Print
        | TokenType::Return => return Ok(()),
        _ => {}
      }
      self.advance_token()?;
    }
    Ok(())
  }

  pub(crate) fn expression_statement(&mut self) -> Result<(), InterpretError> {
    self.expression()?;
    self.consume_token(TokenType::Semicolon, "Expect `;` after expression.".into())?;
    self.emit_byte(OpCode::Pop as u8)
  }
}
