//! # Common
//!
//! Common types and functions used throughout the library.

#![allow(dead_code)]

use crate::utils::Init;

/// ## Value
///
/// A type alias for the value used in the virtual machine.
pub type Value = f64;

/// ## ValueArray
///
/// A struct which represents a sequence of values.
#[derive(Debug, Default, Clone)]
pub struct ValueArray {
  pub(crate) values: Vec<Value>,
}

impl ValueArray {
  /// Write a value to the given value_array.
  pub fn write(&mut self, value: Value) {
    self.values.push(value);
  }

  /// Clear the given value_array.
  pub fn clear(&mut self) {
    self.values.clear();
  }
}

impl Init for ValueArray {}
