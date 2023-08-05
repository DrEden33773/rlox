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
#[repr(C)]
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
  /// Make a token, specifically from `string`.
  fn string(&mut self) -> Token<'a> {
    // Try finding the closing quote.
    while self.peek() != b'"' && !self.is_at_end() {
      if self.peek() == b'\n' {
        self.line += 1;
      }
      self.advance();
    }

    // Cannot find the closing quote.
    if self.is_at_end() {
      return self.error_token("Unterminated string.");
    }

    self.advance();
    self.make_token(TokenType::String)
  }

  /// Make a token, specifically from `number`.
  fn number(&mut self) -> Token<'a> {
    while self.peek().is_ascii_digit() {
      self.advance();
    }

    // Seeking for a fractional part
    if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
      // Consume the "."
      self.advance();

      // Consume the fractional part
      while self.peek().is_ascii_digit() {
        self.advance();
      }
    }

    self.make_token(TokenType::Number)
  }
}

impl<'a> Scanner<'a> {
  /// Make a token.
  fn make_token(&self, token_type: TokenType) -> Token<'a> {
    Token {
      token_type,
      line: self.line,
      lexeme: &self.source[self.start..self.current],
    }
  }

  /// Make an error token.
  fn error_token(&self, message: &'a str) -> Token<'a> {
    Token {
      token_type: TokenType::Error,
      line: self.line,
      lexeme: message,
    }
  }
}

impl<'a> Scanner<'a> {
  /// Scan token from scanner
  pub fn scan_token(&mut self) -> Token<'a> {
    self.skip_white_space();

    // reset start position
    self.start = self.current;

    if self.is_at_end() {
      return self.make_token(TokenType::Eof);
    }

    let c = self.advance();

    if c.is_ascii_digit() {
      return self.number();
    }

    match c {
      // mono-character tokens
      b'(' => self.make_token(TokenType::LeftParen),
      b')' => self.make_token(TokenType::RightParen),
      b'{' => self.make_token(TokenType::LeftBrace),
      b'}' => self.make_token(TokenType::RightBrace),
      b';' => self.make_token(TokenType::Semicolon),
      b',' => self.make_token(TokenType::Comma),
      b'.' => self.make_token(TokenType::Dot),
      b'-' => self.make_token(TokenType::Minus),
      b'+' => self.make_token(TokenType::Plus),
      b'/' => self.make_token(TokenType::Slash),
      b'*' => self.make_token(TokenType::Star),
      // possible two-character tokens
      b'!' => {
        if self.match_next(b'=') {
          self.make_token(TokenType::BangEqual)
        } else {
          self.make_token(TokenType::Bang)
        }
      }
      b'=' => {
        if self.match_next(b'=') {
          self.make_token(TokenType::EqualEqual)
        } else {
          self.make_token(TokenType::Equal)
        }
      }
      b'<' => {
        if self.match_next(b'=') {
          self.make_token(TokenType::LessEqual)
        } else {
          self.make_token(TokenType::Less)
        }
      }
      b'>' => {
        if self.match_next(b'=') {
          self.make_token(TokenType::GreaterEqual)
        } else {
          self.make_token(TokenType::Greater)
        }
      }
      // string
      b'"' => self.string(),
      _ => self.error_token("Unexpected character."),
    }
  }
}

impl<'a> Scanner<'a> {
  /// Check if the scanner is at the end of the source code.
  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  /// Get current char, then advance the scanner (one step).
  fn advance(&mut self) -> u8 {
    self.current += 1;
    self.source.as_bytes()[self.current - 1]
  }

  /// Check if the next char matches the expected char.
  ///
  /// If it matches, advance the scanner (one step) immediately.
  fn match_next(&mut self, expected: u8) -> bool {
    if self.is_at_end() {
      return false;
    }
    if self.source.as_bytes()[self.current] != expected {
      return false;
    }
    self.current += 1;
    true
  }

  /// Get current char, without advancing the scanner.
  fn peek(&self) -> u8 {
    if self.is_at_end() {
      return b'\0';
    }
    self.source.as_bytes()[self.current]
  }

  /// Get the next char, without advancing the scanner.
  fn peek_next(&self) -> u8 {
    if self.current + 1 >= self.source.len() {
      return b'\0';
    }
    self.source.as_bytes()[self.current + 1]
  }

  fn skip_white_space(&mut self) {
    loop {
      let c = self.peek();
      match c {
        b' ' | b'\r' | b'\t' => {
          self.advance();
        }
        b'\n' => {
          self.line += 1;
          self.advance();
        }
        b'/' => {
          if self.peek_next() == b'/' {
            while self.peek() != b'\n' && !self.is_at_end() {
              self.advance();
            }
          } else {
            return;
          }
        }
        _ => return,
      }
    }
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
