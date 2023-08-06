//! # Compiler
//!
//! The compiler is responsible for:
//!
//! - compiling source_code into bytecode
//!
//! It is responsible for executing the bytecode.

use crate::{
  scanner::{Scanner, Token, TokenType},
  utils::Init,
  vm::{InterpretError, VM},
};

#[derive(Default)]
pub struct Parser {
  /// Current token.
  pub(crate) current: Token,
  /// Previous token.
  pub(crate) previous: Token,
  /// If had error.
  pub(crate) had_error: bool,
  /// If in panic mode.
  pub(crate) panic_mode: bool,
}

impl Init for Parser {}

impl Parser {
  fn advance(&mut self, scanner: &mut Scanner) -> Result<(), InterpretError> {
    self.previous = self.current.clone();
    loop {
      self.current = scanner.scan_token();
      if self.current.token_type != TokenType::Error {
        break;
      }
      self.error_at_current(self.current.lexeme.to_owned())?;
    }
    Ok(())
  }

  fn consume(
    &mut self,
    scanner: &mut Scanner,
    token_type: TokenType,
    message: String,
  ) -> Result<(), InterpretError> {
    if self.current.token_type == token_type {
      self.advance(scanner)?;
      return Ok(());
    }
    self.error_at_current(message)
  }
}

impl Parser {
  fn error_at_current(&mut self, message: String) -> Result<(), InterpretError> {
    self.error_at(true, message)
  }

  #[allow(dead_code)]
  fn error(&mut self, message: String) -> Result<(), InterpretError> {
    self.error_at(false, message)
  }

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

impl VM {
  pub(crate) fn compile(&mut self, src: String) -> Result<(), InterpretError> {
    let mut scanner = Scanner::bind(src);
    let mut parser = Parser::init();
    parser.advance(&mut scanner)?;
    // parser.expression(&mut scanner)?;
    parser.consume(
      &mut scanner,
      TokenType::Eof,
      "Expect end of expression.".into(),
    )?;
    Ok(())
  }

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

impl VM {
  fn emit_byte(&mut self, parser: &Parser, byte: u8) {
    self.chunk.write_chunk(byte, parser.previous.line);
  }
}
