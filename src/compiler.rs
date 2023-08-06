//! # Compiler
//!
//! The compiler is responsible for:
//!
//! - compiling source_code into bytecode
//!
//! It is responsible for executing the bytecode.

use crate::{
  chunk::{Chunk, OpCode},
  common::Value,
  scanner::{Scanner, Token, TokenType},
  utils::Init,
  vm::{InterpretError, VM},
};

use lazy_static::lazy_static;
use std::collections::HashMap;

/// ## Precedence
///
/// An enum which represents the precedence of the operators.
///
/// These are all of Lox’s precedence levels,
/// in order from lowest to highest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, enum_repr::EnumU8)]
pub enum Precedence {
  None,       // No precedence (reserved for errors)
  Assignment, // =
  Or,         // or
  And,        // and
  Equality,   // == !=
  Comparison, // < > <= >=
  Term,       // + -
  Factor,     // * /
  Unary,      // ! -
  Call,       // . ()
  Primary,    // literals, grouping, identifiers, this, super
}
impl Precedence {
  /// Get the next precedence.
  ///
  /// Next precedence => a precedence which is higher than the current one.
  fn next(&self) -> Precedence {
    match self {
      Self::Primary => Self::Primary,
      _ => (*self as u8 + 1).into(),
    }
  }
}

impl Default for Precedence {
  fn default() -> Self {
    Self::None
  }
}

type ParseFn = fn(&mut Parser) -> Result<(), InterpretError>;

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

lazy_static! {
  /// ## RULES_VEC
  ///
  /// A vec which contains the rules for the different tokens.
  ///
  /// Initialized by lazy_static!.
  static ref RULES_VEC: Vec<(TokenType, ParseRule)> = vec![
    (
      TokenType::LeftParen,
      ParseRule::new(Some(Parser::grouping), None, Precedence::None)
    ),
    (
      TokenType::RightParen,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::LeftBrace,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::RightBrace,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Comma,
      ParseRule::new(None, None, Precedence::None)
    ),
    (TokenType::Dot, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::Minus,
      ParseRule::new(Some(Parser::unary), Some(Parser::binary), Precedence::Term)
    ),
    (
      TokenType::Plus,
      ParseRule::new(None, Some(Parser::binary), Precedence::Term)
    ),
    (
      TokenType::Semicolon,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Slash,
      ParseRule::new(None, Some(Parser::binary), Precedence::Factor)
    ),
    (
      TokenType::Star,
      ParseRule::new(None, Some(Parser::binary), Precedence::Factor)
    ),
    (
      TokenType::Bang,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::BangEqual,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Equal,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::EqualEqual,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Greater,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::GreaterEqual,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Less,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::LessEqual,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Identifier,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::String,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Number,
      ParseRule::new(Some(Parser::number), None, Precedence::None)
    ),
    (TokenType::And, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::Class,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Else,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::False,
      ParseRule::new(None, None, Precedence::None)
    ),
    (TokenType::For, ParseRule::new(None, None, Precedence::None)),
    (TokenType::Fun, ParseRule::new(None, None, Precedence::None)),
    (TokenType::If, ParseRule::new(None, None, Precedence::None)),
    (TokenType::Nil, ParseRule::new(None, None, Precedence::None)),
    (TokenType::Or, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::Print,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Return,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Super,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::This,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::True,
      ParseRule::new(None, None, Precedence::None)
    ),
    (TokenType::Var, ParseRule::new(None, None, Precedence::None)),
    (
      TokenType::While,
      ParseRule::new(None, None, Precedence::None)
    ),
    (
      TokenType::Error,
      ParseRule::new(None, None, Precedence::None)
    ),
    (TokenType::Eof, ParseRule::new(None, None, Precedence::None)),
  ];
}

lazy_static! {
  /// ## RULES
  ///
  /// HashMap form of `RULES_VEC`
  ///
  /// Initialized by lazy_static!.
  static ref RULES: HashMap<TokenType, ParseRule> = {
    let mut map = HashMap::new();
    for (token_type, rule) in RULES_VEC.iter() {
      map.insert(*token_type, *rule);
    }
    map
  };
}

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
  // If had error.
  // pub(crate) had_error: bool,
  // If in panic mode.
  // pub(crate) panic_mode: bool,
}

impl Init for Parser {}

impl Parser {
  fn number(&mut self) -> Result<(), InterpretError> {
    match self.previous.lexeme.parse::<f64>() {
      Ok(value) => self.emit_constant(value),
      Err(_) => Err(InterpretError::CompileError(
        "Failed to parse number(value).".into(),
      )),
    }
  }

  fn unary(&mut self) -> Result<(), InterpretError> {
    let operator_type = self.previous.token_type;

    // Compile the operand
    self.parse_precedence(Precedence::Unary)?;

    // Emit the operator instruction
    match operator_type {
      TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
      _ => Err(InterpretError::CompileError(
        "Unknown unary operator.".into(),
      )),
    }
  }

  fn binary(&mut self) -> Result<(), InterpretError> {
    let operator_type = self.previous.token_type;
    let rule = self.get_rule(operator_type);
    self.parse_precedence(rule.precedence.next())?;

    match operator_type {
      TokenType::Plus => self.emit_byte(OpCode::Add as u8),
      TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
      TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
      TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
      _ => Err(InterpretError::CompileError(
        "Unknown unary operator.".into(),
      )),
    }
  }

  fn grouping(&mut self) -> Result<(), InterpretError> {
    self.expression()?;
    self.consume(
      TokenType::RightParen,
      "Expect ')' after expression.".to_owned(),
    )
  }
}

impl Parser {
  /// This function starts at the current token,
  /// then parses any expression at the given precedence level or higher.
  fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), InterpretError> {
    // look up `prefix parser` for the `current` token
    self.advance()?;
    let prefix_rule = self.get_rule(self.previous.token_type).prefix;
    if prefix_rule.is_none() {
      return Err(InterpretError::CompileError("Expect expression.".into()));
    }
    let prefix_rule = prefix_rule.unwrap();
    prefix_rule(self)?;

    // look for `infix parser` for the `next` token.
    while precedence <= self.get_rule(self.current.token_type).precedence {
      self.advance()?;
      let infix_rule = self.get_rule(self.previous.token_type).infix;
      if infix_rule.is_none() {
        // no infix rule, so we are done
        break;
      }
      infix_rule.unwrap()(self)?;
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
  fn advance(&mut self) -> Result<(), InterpretError> {
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
  fn consume(&mut self, token_type: TokenType, message: String) -> Result<(), InterpretError> {
    if self.current.token_type == token_type {
      self.advance()?;
      Ok(())
    } else {
      self.error_at_current(message)
    }
  }

  /// Parse the expression.
  fn expression(&mut self) -> Result<(), InterpretError> {
    self.parse_precedence(Precedence::Assignment)
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
  fn end_compiler(&mut self) -> Result<(), InterpretError> {
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
    // if self.panic_mode { return Ok(()); }
    // self.panic_mode = true;
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
    // self.had_error = true;
    Err(InterpretError::CompileError(error_str))
  }
}

impl VM {
  /// This function will compile the source code into bytecode.
  ///
  /// Chunk which holds bytecode sequence is created by `Parser`,
  /// and then loaded(moved) into VM.
  pub(crate) fn compile(&mut self, src: String) -> Result<(), InterpretError> {
    // parse
    let mut parser = Parser::init();
    parser.scanner.rebind(src);
    parser.advance()?;
    parser.expression()?;
    parser.consume(TokenType::Eof, "Expect end of expression.".into())?;
    // manually end compiler
    parser.end_compiler()?;
    // load pre-parsed chunk into VM (link to VM)
    self.chunk = parser.chunk;
    Ok(())
  }

  /// This function is used for debugging.
  ///
  /// It will only compile to token, skipping `parsing`
  pub(crate) fn compile_to_token(&mut self, src: String) -> Result<(), InterpretError> {
    let mut scanner = Scanner::bind(src);
    let mut line = 0_usize;
    loop {
      let token = scanner.scan_token();
      if token.line != line {
        print!("{:>4} ", token.line);
        line = token.line;
      } else {
        print!("   | ");
      }
      println!("[{:?}] '{}'", token.token_type, token.lexeme);
      match token.token_type {
        TokenType::Eof | TokenType::Error => break,
        _ => (),
      }
    }
    Ok(())
  }
}
