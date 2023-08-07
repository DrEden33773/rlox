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
    fn hash_once(rust_string: &str) -> usize {
      let mut hash = 2166136261;
      for byte in rust_string.bytes() {
        hash ^= byte as usize;
        hash = hash.wrapping_mul(16777619);
      }
      hash
    }
    let hash = hash_once(&rust_string);
    Self {
      obj: Obj::new(ObjType::String),
      data: rust_string,
      hash,
    }
  }
}

impl From<ObjString> for String {
  fn from(obj_string: ObjString) -> Self {
    obj_string.data
  }
}
