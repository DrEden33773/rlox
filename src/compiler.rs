//! # Compiler
//!
//! The compiler is responsible for:
//!
//! - compiling source_code into bytecode
//!
//! It is responsible for executing the bytecode.

use crate::{
  scanner::{Scanner, Token, TokenType},
  vm::{InterpretError, VM},
};

#[derive(Default)]
pub struct Parser {
  /// Current token.
  pub(crate) current: Token,
  /// Previous token.
  pub(crate) previous: Token,
  pub(crate) had_error: bool,
}

impl Parser {
  fn error_at_current(&mut self, message: &str) {
    self.error_at(true, message);
  }

  fn error(&mut self, message: &str) {
    self.error_at(false, message);
  }

  fn error_at(&mut self, if_current: bool, message: &str) {
    let token = if if_current {
      &self.current
    } else {
      &self.previous
    };
    eprint!("[line {}] Error", token.line);
    match token.token_type {
      TokenType::Eof => eprint!(" at end"),
      TokenType::Error => {}
      _ => eprint!(" at '{}'", token.lexeme),
    }
    eprintln!(": {}", message);
    self.had_error = true;
  }
}

impl VM {
  pub(crate) fn compile(&mut self, src: String) -> Result<(), InterpretError> {
    let mut _scanner = Scanner::bind(src);
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
