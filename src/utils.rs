//! # Utils
//!
//! This module contains utility components (functions, traits, etc.) for the project.

pub trait Init: Default {
  /// Creates a new instance of the given type.
  fn init() -> Self {
    Self::default()
  }
}
