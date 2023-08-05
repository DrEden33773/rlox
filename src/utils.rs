//! # Utils
//!
//! This module contains utility components (functions, traits, etc.) for the project.

#![allow(dead_code)]

/// A trait for initializing a type.
pub trait Init: Default {
  /// Creates a new instance of the given type.
  fn init() -> Self {
    Self::default()
  }
}

/// Get the command line arguments (without first `root` path argument).
///
/// We have to rewrite this, as [`std::env::args`]'s first argument
/// is always the path to the executable
pub fn args_without_root() -> Vec<String> {
  use std::env::args;
  let args = args().collect::<Vec<_>>();
  let (_, args) = args.split_first().unwrap();
  args.to_vec()
}

/// Get the command line arguments (with first `root` path argument).
///
/// A simple wrapper for [`std::env::args`].
pub fn args() -> Vec<String> {
  use std::env::args;
  args().collect::<Vec<_>>()
}

/// A trait for checking if a type is a valid part of an identifier.
pub trait Identifier {
  fn is_ascii_identifier(&self) -> bool;
}

impl Identifier for u8 {
  /// Check if a byte is a valid part of an identifier.
  fn is_ascii_identifier(&self) -> bool {
    matches!(self, b'a'..=b'z' | b'A'..=b'Z' | b'_')
  }
}
