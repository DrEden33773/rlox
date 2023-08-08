use super::*;

impl Parser {
  pub(crate) fn number_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.number()
  }

  pub(crate) fn number(&mut self) -> Result<(), InterpretError> {
    match self.previous.lexeme.parse::<f64>() {
      Ok(value) => self.emit_constant(value.into()),
      Err(_) => Err(InterpretError::CompileError(
        "Failed to parse number(value).".into(),
      )),
    }
  }

  pub(crate) fn string_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.string()
  }

  pub(crate) fn string(&mut self) -> Result<(), InterpretError> {
    let len = self.previous.lexeme.len();
    let rust_string = self.previous.lexeme[1..len - 1].to_owned();
    let obj_string = ObjString::from(rust_string);
    let obj = obj_string.cast_to_obj_ptr();
    self.emit_constant(Value::obj_val(obj))
  }

  pub(crate) fn named_variable(&mut self, can_assign: bool) -> Result<(), InterpretError> {
    let arg = self.resolve_local()?;
    let (arg, get_op, set_op) = if let Some(arg) = arg {
      (arg as u8, OpCode::GetLocal, OpCode::SetLocal)
    } else {
      (
        self.identifier_constant()?,
        OpCode::GetGlobal,
        OpCode::SetGlobal,
      )
    };
    if can_assign && self.match_token(TokenType::Equal)? {
      self.expression()?;
      self.emit_bytes(&[set_op as u8, arg])
    } else {
      self.emit_bytes(&[get_op as u8, arg])
    }
  }

  pub(crate) fn variable(&mut self, can_assign: bool) -> Result<(), InterpretError> {
    self.named_variable(can_assign)
  }

  pub(crate) fn unary_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.unary()
  }

  pub(crate) fn unary(&mut self) -> Result<(), InterpretError> {
    let operator_type = self.previous.token_type;

    // Compile the operand
    self.parse_precedence(Precedence::Unary)?;

    // Emit the operator instruction
    match operator_type {
      TokenType::Bang => self.emit_byte(OpCode::Not as u8),
      TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
      _ => Err(InterpretError::CompileError(
        "Unknown unary operator.".into(),
      )),
    }
  }

  pub(crate) fn binary_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.binary()
  }

  pub(crate) fn binary(&mut self) -> Result<(), InterpretError> {
    let operator_type = self.previous.token_type;
    let rule = self.get_rule(operator_type);
    self.parse_precedence(rule.precedence.next())?;

    match operator_type {
      TokenType::BangEqual => self.emit_byte(OpCode::NotEqual as u8),
      TokenType::EqualEqual => self.emit_byte(OpCode::Equal as u8),
      TokenType::Greater => self.emit_byte(OpCode::Greater as u8),
      TokenType::GreaterEqual => self.emit_byte(OpCode::GreaterEqual as u8),
      TokenType::Less => self.emit_byte(OpCode::Less as u8),
      TokenType::LessEqual => self.emit_byte(OpCode::LessEqual as u8),
      TokenType::Plus => self.emit_byte(OpCode::Add as u8),
      TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
      TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
      TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
      _ => Err(InterpretError::CompileError(
        "Unknown unary operator.".into(),
      )),
    }
  }

  pub(crate) fn literal_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.literal()
  }

  pub(crate) fn literal(&mut self) -> Result<(), InterpretError> {
    match self.previous.token_type {
      TokenType::False => self.emit_byte(OpCode::False as u8),
      TokenType::Nil => self.emit_byte(OpCode::Nil as u8),
      TokenType::True => self.emit_byte(OpCode::True as u8),
      _ => Err(InterpretError::CompileError(
        "Unknown literal operator.".into(),
      )),
    }
  }

  pub(crate) fn grouping_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.grouping()
  }

  pub(crate) fn grouping(&mut self) -> Result<(), InterpretError> {
    self.expression()?;
    self.consume_token(
      TokenType::RightParen,
      "Expect `)` after expression.".to_owned(),
    )
  }
}
