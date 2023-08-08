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

use self::parser::Parser;

pub mod parser;

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

/// ## Local
///
/// A struct which represents a local variable.
#[derive(Debug, Default, Clone)]
pub struct Local {
  /// The name of the local variable.
  pub(crate) name: Token,
  /// The depth of the local variable.
  pub(crate) depth: usize,
  /// If the bind of local variable initialized.
  pub(crate) is_initialized: bool,
}

/// ## Compiler
///
/// A struct which represents the compiler.
///
/// Mainly deal with `local variables` via a flat array of all locals.
#[derive(Debug, Clone)]
pub struct Compiler {
  /// Represents all locals.
  pub(crate) locals: Vec<Local>,
  /// Tracks how many locals are in scope.
  /// Or to say, track how many of those array slots are in use.
  pub(crate) local_count: usize,
  /// Tracks the number of blocks surrounding the current bit of code
  pub(crate) scope_depth: usize,
}

impl Default for Compiler {
  fn default() -> Self {
    Self {
      locals: vec![Local::default(); u8::MAX as usize + 1],
      local_count: 0,
      scope_depth: 0,
    }
  }
}

impl Init for Compiler {}

impl VM {
  /// This function will compile the source code into bytecode.
  ///
  /// Chunk which holds bytecode sequence is created by `Parser`,
  /// and then loaded(moved) into VM.
  pub(crate) fn compile(&mut self, src: String) -> Result<(), InterpretError> {
    // parse
    let mut parser = Parser::init();
    parser.scanner.rebind(src);
    parser.advance_token()?;
    while !parser.match_token(TokenType::Eof)? {
      parser.declaration()?;
    }
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
