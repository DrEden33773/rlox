//! # Object
//!
//! This module mainly defines the `Object` type, which is the type of the
//! objects in the virtual machine.

use crate::{
  value::{Value, ValueType},
  vm::InterpretError,
};
use std::{fmt::Debug, ptr::NonNull};

/// ## Object Type
///
/// An enum which represents the type of the object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjType {
  String,
}

/// ## Object
///
/// The meta type of all `objects` in the virtual machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Obj {
  pub(crate) obj_type: ObjType,
}

impl Obj {
  pub fn new(obj_type: ObjType) -> Self {
    Self { obj_type }
  }
}

impl Value {
  pub fn is_obj(&self) -> bool {
    self.value_type == ValueType::Obj
  }

  pub fn as_obj(&self) -> Option<NonNull<Obj>> {
    unsafe { self.val_union.obj }
  }
}

impl Value {
  pub fn obj_type(&self) -> Result<ObjType, InterpretError> {
    match self.value_type {
      ValueType::Obj => match unsafe { self.val_union.obj } {
        Some(obj) => Ok(unsafe { obj.as_ref().obj_type }),
        None => Err(InterpretError::RuntimeError(
          "Object is `Nil`, but not recognized as `ValueType::Nil`.".into(),
        )),
      },
      ValueType::Nil => Err(InterpretError::RuntimeError("Value is `nil`.".into())),
      _ => Err(InterpretError::RuntimeError(
        "Value is not an object.".into(),
      )),
    }
  }

  fn is_obj_type(&self, expect: ObjType) -> bool {
    self.is_obj() && unsafe { self.as_obj().unwrap().as_ref() }.obj_type == expect
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ObjString {
  pub(crate) obj: Obj,
  pub(crate) data: String,
}

impl Value {
  pub fn is_string(&self) -> bool {
    self.is_obj_type(ObjType::String)
  }

  pub fn as_string(&self) -> Option<NonNull<ObjString>> {
    if self.is_string() {
      Some(self.as_obj().unwrap().cast())
    } else {
      None
    }
  }

  pub fn as_rust_string(&self) -> Option<&mut String> {
    if let Some(mut obj_string) = self.as_string() {
      Some(&mut unsafe { obj_string.as_mut() }.data)
    } else {
      None
    }
  }
}
