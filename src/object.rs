//! # Object
//!
//! This module mainly defines the `Object` type, which is the type of the
//! objects in the virtual machine.

use crate::{
  value::{ValUnion, Value, ValueType},
  vm::InterpretError,
};
use std::{
  fmt::{Debug, Display},
  ptr::NonNull,
};

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
#[derive(Debug, PartialEq, Eq)]
pub struct Obj {
  pub(crate) obj_type: ObjType,
}

impl Obj {
  pub fn new(obj_type: ObjType) -> Self {
    Self { obj_type }
  }
}

/// ## Object Trait
///
/// A helper trait of meta type `Object`
pub trait ObjTrait {
  fn cast_to_obj_ptr(self) -> NonNull<Obj>;
}

impl Value {
  pub(crate) fn format_object(&self) -> String {
    match self.obj_type().unwrap() {
      ObjType::String => format!("\"{}\"", self.as_rust_string().unwrap()),
    }
  }
}

impl Value {
  pub fn obj_val(value: NonNull<Obj>) -> Self {
    Self {
      value_type: ValueType::Obj,
      val_union: ValUnion { obj: value },
    }
  }

  pub fn is_obj(&self) -> bool {
    self.value_type == ValueType::Obj
  }

  pub fn as_obj(&self) -> NonNull<Obj> {
    unsafe { self.val_union.obj }
  }
}

impl Value {
  pub fn obj_type(&self) -> Result<ObjType, InterpretError> {
    match self.value_type {
      ValueType::Obj => Ok(unsafe { self.val_union.obj.as_ref() }.obj_type),
      ValueType::Nil => Err(InterpretError::RuntimeError("Value is `nil`.".into())),
      _ => Err(InterpretError::RuntimeError(
        "Value is not an object.".into(),
      )),
    }
  }

  fn is_obj_type(&self, expect: ObjType) -> bool {
    self.is_obj() && self.obj_type().unwrap() == expect
  }
}

/// ## Object String
///
/// The type of the string object.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct ObjString {
  pub(crate) obj: Obj,
  pub(crate) data: String,
}

impl ObjTrait for ObjString {
  fn cast_to_obj_ptr(self) -> NonNull<Obj> {
    // NonNull::from(&self).cast() /* => Error! */
    NonNull::new(Box::into_raw(Box::new(self))).unwrap().cast() /* => Ok */
  }
}

impl Display for ObjString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.data)
  }
}

impl Value {
  pub fn is_string(&self) -> bool {
    self.is_obj_type(ObjType::String)
  }

  pub fn as_string(&self) -> Result<NonNull<ObjString>, InterpretError> {
    if self.is_string() {
      Ok(self.as_obj().cast())
    } else {
      Err(InterpretError::RuntimeError(
        "Value is not a string.".into(),
      ))
    }
  }

  pub fn as_rust_string(&self) -> Result<&mut String, InterpretError> {
    let str_ref = &mut unsafe { self.as_string()?.as_mut() }.data;
    Ok(str_ref)
  }
}
