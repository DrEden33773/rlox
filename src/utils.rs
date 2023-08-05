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

/// Get the command line arguments
///
/// We have to rewrite this, as [`std::env::args`]'s first argument
/// is always the path to the executable
pub fn get_args() -> Vec<String> {
  use std::env::args;
  let args = args().collect::<Vec<_>>();
  let (_, args) = args.split_first().unwrap();
  args.to_vec()
}
