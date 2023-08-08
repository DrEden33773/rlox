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

use super::*;

type ParseFn = fn(&mut Parser, bool) -> Result<(), InterpretError>;

pub mod compile_time_error_handlers;
pub mod emit_methods;
pub mod ops_after_get_parse_rule;
pub mod statement_methods;
pub mod variable_methods;

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
    } else if self.match_token(TokenType::If)? {
      self.if_statement()
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
