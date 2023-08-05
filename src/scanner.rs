//! # Scanner
//!
//! The scanner is responsible for:
//!
//! - reading the source code
//! - producing a stream of tokens.

use crate::utils::Init;

/// ## TokenType
///
/// An enum which represents the different types of tokens.
#[repr(u8)]
#[derive(Debug, enum_repr::EnumU8, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,
  // One or two character tokens.
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  // Literals.
  Identifier,
  String,
  Number,
  // Keywords.
  And,
  Class,
  Else,
  False,
  For,
  Fun,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
  // Error and EOF.
  Error,
  Eof,
}

/// ## Token
///
/// A struct which represents a token.
pub struct Token<'a> {
  /// The type of the token.
  pub(crate) token_type: TokenType,
  /// The line of the token.
  pub(crate) line: usize,
  /// The lexeme of the token.
  ///
  /// A lexeme is the text that the token represents.
  pub(crate) lexeme: &'a str,
}

impl<'a> Init for Token<'a> {}

impl<'a> Default for Token<'a> {
  fn default() -> Self {
    Self {
      token_type: TokenType::Eof,
      line: 1,
      lexeme: "",
    }
  }
}

/// ## Scanner
///
/// A struct which represents a scanner.
#[derive(Debug)]
pub struct Scanner<'a> {
  /// The source code.
  pub(crate) source: &'a str,
  /// The start position.
  pub(crate) start: usize,
  /// The current position.
  pub(crate) current: usize,
  /// The current line.
  pub(crate) line: usize,
}

impl<'a> Scanner<'a> {
  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn make_token(&self, token_type: TokenType) -> Token<'a> {
    Token {
      token_type,
      line: self.line,
      lexeme: &self.source[self.start..self.current],
    }
  }

  fn error_token(&self, message: &'a str) -> Token<'a> {
    Token {
      token_type: TokenType::Error,
      line: self.line,
      lexeme: message,
    }
  }

  pub fn scan_token(&mut self) -> Token<'a> {
    self.start = self.current;
    if self.is_at_end() {
      return self.make_token(TokenType::Eof);
    }
    self.error_token("Unexpected character.")
  }
}

impl<'a> Scanner<'a> {
  /// Bind a new scanner to the source code.
  #[inline]
  pub fn init(src: &'a str) -> Self {
    Self {
      source: src,
      start: 0,
      current: 0,
      line: 1,
    }
  }

  /// Bind a new scanner to the source code.
  #[inline]
  pub fn bind(src: &'a str) -> Self {
    Scanner::init(src)
  }
}
