//! # Memory
//!
//! This module is mainly about memory management.
//!
//! You could also find functions which manage:
//!
//! - transformation between `rust_defined_types` and `obj_types`

use crate::object::{Obj, ObjString, ObjType};

impl From<String> for ObjString {
  fn from(rust_string: String) -> Self {
    Self {
      obj: Obj::new(ObjType::String),
      data: rust_string,
    }
  }
}

impl From<ObjString> for String {
  fn from(obj_string: ObjString) -> Self {
    obj_string.data
  }
}
