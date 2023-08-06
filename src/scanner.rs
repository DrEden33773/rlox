//! # Scanner
//!
//! The scanner is responsible for:
//!
//! - reading the source code
//! - producing a stream of tokens.
//!
//! Scanner (aka. Lexer) is the first stage of the interpreter,
//! which is also know as `front-end`.

use std::hash::Hash;

use crate::utils::{Identifier, Init};

/// ## TokenType
///
/// An enum which represents the different types of tokens.
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
  // Dollar sign.
  Dollar,
  // Error and EOF.
  Error,
  Eof,
}

impl Hash for TokenType {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    core::mem::discriminant(self).hash(state);
  }
}

// TODO: Add support of `dollar` sign => "var = ${var}".

/// ## Token
///
/// A struct which represents a token.
#[derive(Debug, Clone)]
pub struct Token {
  /// The type of the token.
  pub(crate) token_type: TokenType,
  /// The line of the token.
  pub(crate) line: usize,
  /// The lexeme of the token.
  ///
  /// A lexeme is the text that the token represents.
  pub(crate) lexeme: String,
}

impl Init for Token {}

impl Default for Token {
  fn default() -> Self {
    Self {
      token_type: TokenType::Eof,
      line: 1,
      lexeme: "".into(),
    }
  }
}

/// ## Scanner
///
/// A struct which represents a scanner.
#[derive(Debug)]
pub struct Scanner {
  /// The source code.
  pub(crate) source: String,
  /// The start position.
  pub(crate) start: usize,
  /// The current position.
  pub(crate) current: usize,
  /// The current line.
  pub(crate) line: usize,
}

impl Default for Scanner {
  fn default() -> Self {
    Self {
      source: "".to_string(),
      start: 0,
      current: 0,
      line: 1,
    }
  }
}

impl Scanner {
  /// Try to match reserved keyword.
  ///
  /// Match `rest` and `&self.source[self.current::-len]`
  fn check_keyword(&self, rest: &str, candidate: TokenType) -> TokenType {
    let len = rest.len();
    // 1. steps from start index to current index `should be equal to` len, or the match must failed
    // 2. if `1.` suits, then check if the rest of the source code is equal to the rest pattern
    if self.current >= len && &self.source[self.current - len..self.current] == rest {
      candidate
    } else {
      TokenType::Identifier
    }
  }

  /// Generate correct identifier token.
  fn identifier_type(&self) -> TokenType {
    match self.source.as_bytes()[self.start] {
      b'a' => self.check_keyword("nd", TokenType::And),
      b'c' => self.check_keyword("lass", TokenType::Class),
      b'e' => self.check_keyword("lse", TokenType::Else),
      b'i' => self.check_keyword("f", TokenType::If),
      b'n' => self.check_keyword("il", TokenType::Nil),
      b'o' => self.check_keyword("r", TokenType::Or),
      b'p' => self.check_keyword("rint", TokenType::Print),
      b'r' => self.check_keyword("eturn", TokenType::Return),
      b's' => self.check_keyword("uper", TokenType::Super),
      b'v' => self.check_keyword("ar", TokenType::Var),
      b'w' => self.check_keyword("hile", TokenType::While),
      b'f' if self.current - self.start > 1 => match self.source.as_bytes()[self.start + 1] {
        b'a' => self.check_keyword("lse", TokenType::False),
        b'o' => self.check_keyword("r", TokenType::For),
        b'u' => self.check_keyword("n", TokenType::Fun),
        _ => TokenType::Identifier,
      },
      b't' if self.current - self.start > 1 => match self.source.as_bytes()[self.start + 1] {
        b'h' => self.check_keyword("is", TokenType::This),
        b'r' => self.check_keyword("ue", TokenType::True),
        _ => TokenType::Identifier,
      },
      _ => TokenType::Identifier,
    }
  }
}

impl Scanner {
  /// Make a token, specifically from `string`.
  fn string(&mut self) -> Token {
    // Try finding the closing quote.
    while self.peek() != b'"' && !self.is_at_end() {
      if self.peek() == b'\n' {
        self.line += 1;
      }
      self.advance_char();
    }

    // Cannot find the closing quote.
    if self.is_at_end() {
      return self.error_token("Unterminated string.".into());
    }

    self.advance_char();
    self.make_token(TokenType::String)
  }

  /// Make a token, specifically from `number`.
  fn number(&mut self) -> Token {
    while self.peek().is_ascii_digit() {
      self.advance_char();
    }

    // Seeking for a fractional part
    if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
      // Consume the "."
      self.advance_char();

      // Consume the fractional part
      while self.peek().is_ascii_digit() {
        self.advance_char();
      }
    }

    self.make_token(TokenType::Number)
  }

  /// Make a token, specifically from `identifier`.
  fn identifier(&mut self) -> Token {
    while matches!(self.peek(), c if c.is_ascii_identifier() || c.is_ascii_digit()) {
      self.advance_char();
    }
    self.make_token(self.identifier_type())
  }
}

impl Scanner {
  /// Make a token.
  fn make_token(&self, token_type: TokenType) -> Token {
    Token {
      token_type,
      line: self.line,
      lexeme: (&self.source[self.start..self.current]).into(),
    }
  }

  /// Make an error token.
  fn error_token(&self, message: String) -> Token {
    Token {
      token_type: TokenType::Error,
      line: self.line,
      lexeme: message,
    }
  }
}

impl Scanner {
  /// Scan token from scanner
  pub fn scan_token(&mut self) -> Token {
    self.skip_white_space();

    // reset start position
    self.start = self.current;

    if self.is_at_end() {
      return self.make_token(TokenType::Eof);
    }

    let c = self.advance_char();

    if c.is_ascii_digit() {
      return self.number();
    }
    if c.is_ascii_identifier() {
      return self.identifier();
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
      _ => self.error_token("Unexpected character.".into()),
    }
  }
}

impl Scanner {
  /// Check if the scanner is at the end of the source code.
  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  /// Get current char, then advance the scanner (one step).
  fn advance_char(&mut self) -> u8 {
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
          self.advance_char();
        }
        b'\n' => {
          self.line += 1;
          self.advance_char();
        }
        b'/' => {
          if self.peek_next() == b'/' {
            while self.peek() != b'\n' && !self.is_at_end() {
              self.advance_char();
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

impl Scanner {
  /// Bind a new scanner to the source code.
  #[inline]
  pub fn init_with(src: String) -> Self {
    Self {
      source: src,
      start: 0,
      current: 0,
      line: 1,
    }
  }

  /// Bind a new scanner to the source code.
  #[inline]
  pub fn bind(src: String) -> Self {
    Scanner::init_with(src)
  }

  pub fn rebind(&mut self, src: String) {
    self.source = src;
    self.start = 0;
    self.current = 0;
    self.line = 1;
  }
}
