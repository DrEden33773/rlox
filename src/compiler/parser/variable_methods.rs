use super::*;

impl Parser {
  fn parse_variable(&mut self, message: String) -> Result<u8, InterpretError> {
    self.consume_token(TokenType::Identifier, message)?;

    // record if it's a local variable (scope_depth > 0)
    self.declare_variable()?;

    // if in local scope, simply exit (with a fake index)
    if self.compiler.scope_depth > 0 {
      return Ok(0);
    }

    self.identifier_constant()
  }

  fn mark_initialized(&mut self) {
    self.compiler.locals[self.compiler.local_count - 1].is_initialized = true;
  }

  fn define_variable(&mut self, global_index: u8) -> Result<(), InterpretError> {
    if self.compiler.scope_depth > 0 {
      self.mark_initialized();
      Ok(())
    } else {
      self.emit_bytes(&[OpCode::DefineGlobal as u8, global_index])
    }
  }

  /// Records the existence of variable (only for locals).
  fn declare_variable(&mut self) -> Result<(), InterpretError> {
    if self.compiler.scope_depth == 0 {
      return Ok(());
    }

    // Detect error => two variables with same name
    // in the same local scope.
    for i in (0..self.compiler.local_count).rev() {
      let local = &self.compiler.locals[i];
      if local.is_initialized && local.depth < self.compiler.scope_depth {
        break;
      }
      if self.previous.lexeme == local.name.lexeme {
        return Err(InterpretError::CompileError(
          "Already a variable with this name in this scope.".into(),
        ));
      }
    }

    self.add_local()
  }

  fn add_local(&mut self) -> Result<(), InterpretError> {
    if self.compiler.local_count > u8::MAX as usize {
      return Err(InterpretError::CompileError(
        "Too many local variables in function(At most: 256).".into(),
      ));
    }
    let local = &mut self.compiler.locals[self.compiler.local_count];
    local.name = self.previous.to_owned();
    local.depth = self.compiler.scope_depth;
    local.is_initialized = false;
    self.compiler.local_count += 1;
    Ok(())
  }

  pub(crate) fn identifier_constant(&mut self) -> Result<u8, InterpretError> {
    self.make_constant(Value::obj_val(
      ObjString::from(self.previous.lexeme.to_owned()).cast_to_obj_ptr(),
    ))
  }

  /// Try to find the local variable in the current scope.
  ///
  /// If find, return the index of the local variable.
  pub(crate) fn resolve_local(&mut self) -> Result<Option<usize>, InterpretError> {
    let pos = self.compiler.locals[..self.compiler.local_count]
      .iter()
      .position(|local| local.name.lexeme == self.previous.lexeme);
    if let Some(pos) = pos {
      if !self.compiler.locals[pos].is_initialized {
        return Err(InterpretError::CompileError(
          "Can't read local variable in its own initializer.".into(),
        ));
      };
    }
    Ok(pos)
  }

  /// Declare: bind a new variable.
  pub(crate) fn var_declaration(&mut self) -> Result<(), InterpretError> {
    let global_index = self.parse_variable("Expect variable name.".into())?;

    if self.match_token(TokenType::Equal)? {
      self.expression()?;
    } else {
      self.emit_byte(OpCode::Nil as u8)?;
    }

    self.consume_token(
      TokenType::Semicolon,
      "Expect `;` after variable declaration.".into(),
    )?;

    self.define_variable(global_index)
  }
}
