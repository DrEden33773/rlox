//! # Memory
//!
//! This module is mainly about memory management.
//!
//! You could also find functions which manage:
//!
//! - transformation between `rust_defined_types` and `obj_types`

use crate::object::{Obj, ObjString, ObjType};

impl From<String> for ObjString {
  fn from(value: String) -> Self {
    Self {
      obj: Obj::new(ObjType::String),
      data: value,
    }
  }
}

impl From<ObjString> for String {
  fn from(value: ObjString) -> Self {
    value.data
  }
}
