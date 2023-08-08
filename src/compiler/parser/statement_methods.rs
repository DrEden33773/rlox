use super::*;

impl Parser {
  pub(crate) fn print_statement(&mut self) -> Result<(), InterpretError> {
    self.expression()?;
    self.consume_token(TokenType::Semicolon, "Expect `;` after value.".into())?;
    self.emit_byte(OpCode::Print as u8)
  }

  pub(crate) fn if_statement(&mut self) -> Result<(), InterpretError> {
    /* condition */
    self.consume_token(TokenType::LeftParen, "Expect `(` after `if`.".into())?;
    self.expression()?;
    self.consume_token(TokenType::RightParen, "Expect `)` after condition.".into())?;

    /* `consume`: if {...} */
    let then_jump = self.emit_jump(OpCode::JumpIfFalse as u8)?;
    // pop top of stack **iff** `condition` is true
    self.emit_byte(OpCode::Pop as u8)?;
    self.statement()?;

    /* patch `if` jump */
    let else_jump = self.emit_jump(OpCode::Jump as u8)?;
    self.patch_jump(then_jump)?;

    /* `consume`: else {...} */
    // pop top of stack **iff** `condition` is false
    self.emit_byte(OpCode::Pop as u8)?;
    if self.match_token(TokenType::Else)? {
      self.statement()?;
    }

    /* patch `else` jump */
    self.patch_jump(else_jump)
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
