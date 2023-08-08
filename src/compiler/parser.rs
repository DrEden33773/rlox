//! # Parser
//!
//! Submodule of `compiler`, which is used to parse the source code into bytecode.

use crate::{
  chunk::{Chunk, OpCode},
  compiler::Precedence,
  object::{ObjString, ObjTrait},
  scanner::{Scanner, Token, TokenType},
  utils::Init,
  value::Value,
  vm::InterpretError,
};

use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::{Compiler, Local};

type ParseFn = fn(&mut Parser, bool) -> Result<(), InterpretError>;

/// ## ParseRule
///
/// A struct which represents the parse rule, with:
///
/// - prefix: the prefix parse function
/// - infix: the infix parse function
/// - precedence: the precedence of the operator
#[derive(Default, Clone, Copy)]
pub struct ParseRule {
  pub prefix: Option<ParseFn>,
  pub infix: Option<ParseFn>,
  pub precedence: Precedence,
}

impl ParseRule {
  pub fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> Self {
    Self {
      prefix,
      infix,
      precedence,
    }
  }
}

/// ## RULES_VEC
///
/// A vec which contains the rules for the different tokens.
///
/// Initialized by lazy_static!.
static RULES_VEC: Lazy<Vec<(TokenType, ParseRule)>> = Lazy::new(|| {
  vec![
    (
      TokenType::LeftParen,
      ParseRule::new(Some(Parser::grouping_adapter), None, Precedence::None),
    ),
    (
      TokenType::RightParen,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::LeftBrace,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::RightBrace,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::Comma,
      ParseRule::new(None, None, Precedence::None),
    ),
    (TokenType::Dot, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::Minus,
      ParseRule::new(
        Some(Parser::unary_adapter),
        Some(Parser::binary_adapter),
        Precedence::Term,
      ),
    ),
    (
      TokenType::Plus,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Term),
    ),
    (
      TokenType::Semicolon,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::Slash,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Factor),
    ),
    (
      TokenType::Star,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Factor),
    ),
    (
      TokenType::Bang,
      ParseRule::new(Some(Parser::unary_adapter), None, Precedence::None),
    ),
    (
      TokenType::BangEqual,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Equality),
    ),
    (
      TokenType::Equal,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::EqualEqual,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Equality),
    ),
    (
      TokenType::Greater,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Comparison),
    ),
    (
      TokenType::GreaterEqual,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Comparison),
    ),
    (
      TokenType::Less,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Comparison),
    ),
    (
      TokenType::LessEqual,
      ParseRule::new(None, Some(Parser::binary_adapter), Precedence::Comparison),
    ),
    (
      TokenType::Identifier,
      ParseRule::new(Some(Parser::variable), None, Precedence::None),
    ),
    (
      TokenType::String,
      ParseRule::new(Some(Parser::string_adapter), None, Precedence::None),
    ),
    (
      TokenType::Number,
      ParseRule::new(Some(Parser::number_adapter), None, Precedence::None),
    ),
    (TokenType::And, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::Class,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::Else,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::False,
      ParseRule::new(Some(Parser::literal_adapter), None, Precedence::None),
    ),
    (TokenType::For, ParseRule::new(None, None, Precedence::None)),
    (TokenType::Fun, ParseRule::new(None, None, Precedence::None)),
    (TokenType::If, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::Nil,
      ParseRule::new(Some(Parser::literal_adapter), None, Precedence::None),
    ),
    (TokenType::Or, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::Print,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::Return,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::Super,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::This,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::True,
      ParseRule::new(Some(Parser::literal_adapter), None, Precedence::None),
    ),
    (TokenType::Var, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::While,
      ParseRule::new(None, None, Precedence::None),
    ),
    (
      TokenType::Error,
      ParseRule::new(None, None, Precedence::None),
    ),
    (TokenType::Eof, ParseRule::new(None, None, Precedence::None)),
  ]
});

/// ## RULES
///
/// HashMap form of `RULES_VEC`
///
/// Initialized by lazy_static!.
static RULES: Lazy<HashMap<TokenType, ParseRule>> = Lazy::new(|| {
  let mut map = HashMap::new();
  for (token_type, rule) in RULES_VEC.iter() {
    map.insert(*token_type, *rule);
  }
  map
});

#[derive(Default)]
pub struct Parser {
  /// Chunk used for compiling.
  pub(crate) chunk: Chunk,
  /// Scanner in parser
  pub(crate) scanner: Scanner,
  /// Current token.
  pub(crate) current: Token,
  /// Previous token.
  pub(crate) previous: Token,
  /// If had error.
  pub(crate) had_error: bool,
  /// If in panic mode.
  pub(crate) panic_mode: bool,
  /// Compiler => handle local variables
  pub(crate) compiler: Compiler,
}

impl Init for Parser {}

impl Parser {
  fn number_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.number()
  }

  fn number(&mut self) -> Result<(), InterpretError> {
    match self.previous.lexeme.parse::<f64>() {
      Ok(value) => self.emit_constant(value.into()),
      Err(_) => Err(InterpretError::CompileError(
        "Failed to parse number(value).".into(),
      )),
    }
  }

  fn string_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.string()
  }

  fn string(&mut self) -> Result<(), InterpretError> {
    let len = self.previous.lexeme.len();
    let rust_string = self.previous.lexeme[1..len - 1].to_owned();
    let obj_string = ObjString::from(rust_string);
    let obj = obj_string.cast_to_obj_ptr();
    self.emit_constant(Value::obj_val(obj))
  }

  fn named_variable(&mut self, can_assign: bool) -> Result<(), InterpretError> {
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

  fn variable(&mut self, can_assign: bool) -> Result<(), InterpretError> {
    self.named_variable(can_assign)
  }

  fn unary_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.unary()
  }

  fn unary(&mut self) -> Result<(), InterpretError> {
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

  fn binary_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.binary()
  }

  fn binary(&mut self) -> Result<(), InterpretError> {
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

  fn literal_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.literal()
  }

  fn literal(&mut self) -> Result<(), InterpretError> {
    match self.previous.token_type {
      TokenType::False => self.emit_byte(OpCode::False as u8),
      TokenType::Nil => self.emit_byte(OpCode::Nil as u8),
      TokenType::True => self.emit_byte(OpCode::True as u8),
      _ => Err(InterpretError::CompileError(
        "Unknown literal operator.".into(),
      )),
    }
  }

  fn grouping_adapter(&mut self, _: bool) -> Result<(), InterpretError> {
    self.grouping()
  }

  fn grouping(&mut self) -> Result<(), InterpretError> {
    self.expression()?;
    self.consume_token(
      TokenType::RightParen,
      "Expect `)` after expression.".to_owned(),
    )
  }
}

impl Parser {
  /// This function starts at the current token,
  /// then parses any expression at the given precedence level or higher.
  fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), InterpretError> {
    // if it's valid to operate `assign`
    let can_assign = precedence <= Precedence::Assignment;

    // look up `prefix parser` for the `current` token
    self.advance_token()?;
    let prefix_rule = self.get_rule(self.previous.token_type).prefix;
    if prefix_rule.is_none() {
      return Err(InterpretError::CompileError("Expect expression.".into()));
    }
    let prefix_rule = prefix_rule.unwrap();
    prefix_rule(self, can_assign)?;

    // look for `infix parser` for the `next` token.
    while precedence <= self.get_rule(self.current.token_type).precedence {
      self.advance_token()?;
      let infix_rule = self.get_rule(self.previous.token_type).infix;
      if infix_rule.is_none() {
        // no infix rule, so we are done
        break;
      }
      infix_rule.unwrap()(self, can_assign)?;
    }

    // Deal with invalid assignment.
    // (E.g. => {...}; a + b = c * d;)
    if can_assign && self.match_token(TokenType::Equal)? {
      return Err(InterpretError::CompileError(
        "Invalid assignment target.".into(),
      ));
    }

    // done
    Ok(())
  }

  fn get_rule(&self, token_type: TokenType) -> ParseRule {
    *RULES.get(&token_type).unwrap()
  }
}

impl Parser {
  /// Activate parser, move to next token (one step).
  ///
  /// It asks the scanner for the next token and stores it for later use.
  ///
  /// Before doing that, it takes the old current token and stashes that in a previous field.
  pub(crate) fn advance_token(&mut self) -> Result<(), InterpretError> {
    self.previous = self.current.clone();
    loop {
      self.current = self.scanner.scan_token();
      if self.current.token_type != TokenType::Error {
        break;
      }
      self.error_at_current(self.current.lexeme.to_owned())?;
    }
    Ok(())
  }

  /// Try consuming current(last) token, if can't, throw error.
  fn consume_token(
    &mut self,
    token_type: TokenType,
    message: String,
  ) -> Result<(), InterpretError> {
    if self.current.token_type == token_type {
      self.advance_token()?;
      Ok(())
    } else {
      self.error_at_current(message)
    }
  }

  /// Check if current token has the same type with expected.
  fn check_token(&mut self, expected_type: TokenType) -> bool {
    self.current.token_type == expected_type
  }

  /// Execute `check_token`.
  ///
  /// If true, advance token with true returned.
  ///
  /// Else, directly return false.
  pub(crate) fn match_token(&mut self, expected_type: TokenType) -> Result<bool, InterpretError> {
    if !self.check_token(expected_type) {
      Ok(false)
    } else {
      self.advance_token().unwrap();
      Ok(true)
    }
  }

  /// Parse the expression.
  fn expression(&mut self) -> Result<(), InterpretError> {
    self.parse_precedence(Precedence::Assignment)
  }

  /// Step into a block
  fn begin_scope(&mut self) {
    self.compiler.scope_depth += 1;
  }

  /// Step out of a block
  fn end_scope(&mut self) -> Result<(), InterpretError> {
    self.compiler.scope_depth -= 1;
    while self.compiler.local_count > 0
      && self.compiler.locals.last().unwrap().depth > self.compiler.scope_depth
    {
      // lifetime of local variable ends here, call pop instruction
      self.emit_byte(OpCode::Pop as u8)?;
      self.compiler.local_count -= 1;
    }
    Ok(())
  }

  /// Parse contents in a block
  fn block(&mut self) -> Result<(), InterpretError> {
    while !self.check_token(TokenType::RightBrace) && !self.check_token(TokenType::Eof) {
      self.declaration()?;
    }
    self.consume_token(TokenType::RightBrace, "Expect `}` after block.".into())
  }

  /// Try matching current token as a declaration.
  pub(crate) fn declaration(&mut self) -> Result<(), InterpretError> {
    if self.match_token(TokenType::Var)? {
      self.var_declaration()?;
    } else {
      self.statement()?;
    }
    if self.panic_mode {
      self.synchronize()
    } else {
      Ok(())
    }
  }

  /// Try matching current token as a statement.
  fn statement(&mut self) -> Result<(), InterpretError> {
    if self.match_token(TokenType::Print)? {
      self.print_statement()
    } else if self.match_token(TokenType::LeftBrace)? {
      self.begin_scope();
      self.block()?;
      self.end_scope()?;
      Ok(())
    } else {
      self.expression_statement()
    }
  }
}

impl Parser {
  fn parse_variable(&mut self, message: String) -> Result<u8, InterpretError> {
    self.consume_token(TokenType::Identifier, message)?;
    // declare on variable
    self.declare_variable()?;

    // if in local scope, simply exit (with a fake index)
    if self.compiler.scope_depth > 0 {
      return Ok(0);
    }

    self.identifier_constant()
  }

  fn mark_initialized(&mut self) {
    self.compiler.locals.last_mut().unwrap().is_captured = true;
  }

  fn define_variable(&mut self, global_index: u8) -> Result<(), InterpretError> {
    if self.compiler.scope_depth > 0 {
      self.mark_initialized();
      Ok(())
    } else {
      self.emit_bytes(&[OpCode::DefineGlobal as u8, global_index])
    }
  }

  fn declare_variable(&mut self) -> Result<(), InterpretError> {
    if self.compiler.scope_depth == 0 {
      return Ok(());
    }

    // Detect error => two variables with same name
    // in the same local scope.
    for local in self
      .compiler
      .locals
      .iter()
      .rev()
      .take(self.compiler.local_count)
    {
      if local.depth < self.compiler.scope_depth {
        break;
      }
      if local.name.lexeme == self.previous.lexeme {
        return Err(InterpretError::CompileError(
          "Already a variable with this name in this scope.".into(),
        ));
      }
    }

    self.add_local()
  }

  fn identifier_constant(&mut self) -> Result<u8, InterpretError> {
    self.make_constant(Value::obj_val(
      ObjString::from(self.previous.lexeme.to_owned()).cast_to_obj_ptr(),
    ))
  }

  fn add_local(&mut self) -> Result<(), InterpretError> {
    if self.compiler.local_count > u8::MAX as usize + 1 {
      return Err(InterpretError::CompileError(
        "Too many local variables in function(At most: 256).".into(),
      ));
    }
    self.compiler.locals.push(Local {
      depth: self.compiler.scope_depth,
      name: self.previous.to_owned(),
      is_captured: false,
    });
    self.compiler.local_count += 1;
    Ok(())
  }

  /// Try to find the local variable in the current scope.
  ///
  /// If find, return the index of the local variable.
  fn resolve_local(&mut self) -> Result<Option<usize>, InterpretError> {
    let pos = self
      .compiler
      .locals
      .iter()
      .take(self.compiler.local_count)
      .position(|local| local.name.lexeme == self.previous.lexeme);
    if let Some(pos) = pos {
      if !self.compiler.locals[pos].is_captured {
        Err(InterpretError::CompileError(
          "Can't read local variable in its own initializer.".into(),
        ))
      } else {
        Ok(Some(pos))
      }
    } else {
      Ok(pos)
    }
  }

  /// Declare bind a new variable.
  fn var_declaration(&mut self) -> Result<(), InterpretError> {
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

impl Parser {
  fn print_statement(&mut self) -> Result<(), InterpretError> {
    self.expression()?;
    self.consume_token(TokenType::Semicolon, "Expect `;` after value.".into())?;
    self.emit_byte(OpCode::Print as u8)
  }

  /// If in panic_mode, then synchronize (for better recognizing what error has occurred).
  ///
  /// Synchronize means that, we will skip tokens indiscriminately
  /// until we reach something that looks like a statement boundary.
  ///
  /// E.g.: class | fun | var | for | if | while | print | return
  fn synchronize(&mut self) -> Result<(), InterpretError> {
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

  fn expression_statement(&mut self) -> Result<(), InterpretError> {
    self.expression()?;
    self.consume_token(TokenType::Semicolon, "Expect `;` after expression.".into())?;
    self.emit_byte(OpCode::Pop as u8)
  }
}

impl Parser {
  /// Emit a constant instruction with the given value.
  ///
  /// If it's position/index is out of range of u8, return error.
  ///
  /// Else, return the index.
  ///
  /// That's because, in this simple language,
  /// `chunk.curr_pos + 1` is the only possible position for a constant index,
  /// which is u8 (0..=255).
  ///
  /// TODO: Wrap the chunk, add support of (1, 2, 4, 8) bytes of peek_next logic.
  fn make_constant(&mut self, value: Value) -> Result<u8, InterpretError> {
    let index = self.chunk.add_constant(value);
    if index > u8::MAX as usize {
      Err(
        self
          .error("Too many constants in one chunk.".to_owned())
          .unwrap_err(),
      )
    } else {
      Ok(index as u8)
    }
  }
}

impl Parser {
  /// Appending a sequence of bytes to the chunk (in order).
  fn emit_bytes(&mut self, bytes: &[u8]) -> Result<(), InterpretError> {
    for &byte in bytes {
      self.emit_byte(byte)?;
    }
    Ok(())
  }

  /// Appending a single byte to the chunk.
  fn emit_byte(&mut self, byte: u8) -> Result<(), InterpretError> {
    self.chunk.write_chunk(byte, self.previous.line);
    Ok(())
  }

  /// Specifically appending the return instruction to the chunk.
  fn emit_return(&mut self) -> Result<(), InterpretError> {
    self.emit_byte(OpCode::Return as u8)
  }

  /// Wrapper for appending `constant` and `index` info to the chunk.
  fn emit_constant(&mut self, value: Value) -> Result<(), InterpretError> {
    let constant_index = self.make_constant(value)?;
    self.emit_bytes(&[OpCode::Constant as u8, constant_index])
  }

  /// Operations after end of compilation.
  pub(crate) fn end_compiler(&mut self) -> Result<(), InterpretError> {
    self.emit_return()
  }
}

impl Parser {
  /// Report error at current token.
  fn error_at_current(&mut self, message: String) -> Result<(), InterpretError> {
    self.error_at(true, message)
  }

  // Report error at previous token.
  fn error(&mut self, message: String) -> Result<(), InterpretError> {
    self.error_at(false, message)
  }

  /// Report error at selected token.
  fn error_at(&mut self, if_current: bool, message: String) -> Result<(), InterpretError> {
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
