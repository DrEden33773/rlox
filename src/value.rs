//! # Value
//!
//! This module contains the `Value` type, which is the main type used in the
//! interpreter.
//!
//! Core implementation is about `union`.
//!
//! Wrappers of `Value` (e.g. `ValueArray`) are also included in this module.

use std::{
  fmt::{Debug, Display},
  ptr::NonNull,
};

use crate::{object::Obj, utils::Init, vm::InterpretError};

/// ## Value
///
/// A type alias for the value used in the virtual machine.
// pub type Value = f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, enum_repr::EnumU8)]
pub enum ValueType {
  Bool,
  Nil,
  Number,
  Obj,
}

/// ## Value Union
///
/// A union which holds all possible representation of a value.
#[repr(C)]
#[derive(Clone, Copy)]
pub union ValUnion {
  pub(crate) boolean: bool,
  pub(crate) number: f64,
  pub(crate) obj: Option<NonNull<Obj>>,
}

impl Display for ValUnion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", unsafe { self.number })
  }
}

impl Debug for ValUnion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    unsafe { f.write_str(&self.number.to_string()) }
  }
}

/// ## Value
///
/// A type which represents the value used in the virtual machine.
///
/// It contains:
///
/// - `value_type`: The type of the value.
/// - `val_union`: The representation in memory of the value.
#[derive(Debug, Clone, Copy)]
pub struct Value {
  pub(crate) value_type: ValueType,
  pub(crate) val_union: ValUnion,
}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    if self.value_type != other.value_type {
      None
    } else {
      match self.value_type {
        ValueType::Bool => self.as_bool().partial_cmp(&other.as_bool()),
        ValueType::Nil => Some(std::cmp::Ordering::Equal),
        ValueType::Number => self.as_number().partial_cmp(&other.as_number()),
        ValueType::Obj => todo!(),
      }
    }
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    if self.value_type != other.value_type {
      false
    } else {
      match self.value_type {
        ValueType::Bool => self.as_bool() == other.as_bool(),
        ValueType::Nil => true,
        ValueType::Number => self.as_number() == other.as_number(),
        ValueType::Obj => todo!(),
      }
    }
  }
}

impl std::ops::Not for Value {
  type Output = Result<Self, InterpretError>;
  fn not(self) -> Self::Output {
    match self.value_type {
      ValueType::Bool => Ok(Self::bool_val(!self.as_bool())),
      ValueType::Nil => Ok(Self::bool_val(true)),
      ValueType::Number => Err(InterpretError::RuntimeError(
        "Operand could only be `boolean` or `nil`.".to_owned(),
      )),
      ValueType::Obj => todo!(),
    }
  }
}

impl std::ops::Neg for Value {
  type Output = Result<Self, InterpretError>;
  fn neg(self) -> Self::Output {
    if self.is_number() {
      Ok(Value::number_val(-self.as_number()))
    } else {
      Err(InterpretError::RuntimeError(
        "Operand must be a number.".to_owned(),
      ))
    }
  }
}
impl std::ops::Add for Value {
  type Output = Result<Self, InterpretError>;
  fn add(self, rhs: Self) -> Self::Output {
    if self.is_number() && rhs.is_number() {
      Ok(Value::number_val(self.as_number() + rhs.as_number()))
    } else {
      Err(InterpretError::RuntimeError(
        "Operands must be numbers.".to_owned(),
      ))
    }
  }
}
impl std::ops::Sub for Value {
  type Output = Result<Self, InterpretError>;
  fn sub(self, rhs: Self) -> Self::Output {
    if self.is_number() && rhs.is_number() {
      Ok(Value::number_val(self.as_number() - rhs.as_number()))
    } else {
      Err(InterpretError::RuntimeError(
        "Operands must be numbers.".to_owned(),
      ))
    }
  }
}
impl std::ops::Mul for Value {
  type Output = Result<Self, InterpretError>;
  fn mul(self, rhs: Self) -> Self::Output {
    if self.is_number() && rhs.is_number() {
      Ok(Value::number_val(self.as_number() * rhs.as_number()))
    } else {
      Err(InterpretError::RuntimeError(
        "Operands must be numbers.".to_owned(),
      ))
    }
  }
}
impl std::ops::Div for Value {
  type Output = Result<Self, InterpretError>;
  fn div(self, rhs: Self) -> Self::Output {
    if self.is_number() && rhs.is_number() {
      Ok(Value::number_val(self.as_number() / rhs.as_number()))
    } else {
      Err(InterpretError::RuntimeError(
        "Operands must be numbers.".to_owned(),
      ))
    }
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.value_type {
      ValueType::Bool => write!(f, "{}", self.as_bool()),
      ValueType::Nil => write!(f, "nil"),
      ValueType::Number => write!(f, "{}", self.as_number()),
      ValueType::Obj => todo!(),
    }
  }
}

impl From<f64> for Value {
  fn from(value: f64) -> Self {
    Self::number_val(value)
  }
}
impl From<Value> for f64 {
  fn from(val: Value) -> Self {
    val.as_number()
  }
}
impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Self::bool_val(value)
  }
}
impl From<Value> for bool {
  fn from(val: Value) -> Self {
    val.as_bool()
  }
}
impl From<Option<f64>> for Value {
  fn from(value: Option<f64>) -> Self {
    match value {
      Some(v) => Self::number_val(v),
      None => Self::nil_val(),
    }
  }
}
impl From<Value> for Option<f64> {
  fn from(val: Value) -> Self {
    match val.value_type {
      ValueType::Number => Some(val.as_number()),
      ValueType::Nil => None,
      _ => panic!("Cannot convert {:?} to Option<f64>.", val),
    }
  }
}
impl From<Option<bool>> for Value {
  fn from(value: Option<bool>) -> Self {
    match value {
      Some(v) => Self::bool_val(v),
      None => Self::nil_val(),
    }
  }
}
impl From<Value> for Option<bool> {
  fn from(val: Value) -> Self {
    match val.value_type {
      ValueType::Bool => Some(val.as_bool()),
      ValueType::Nil => None,
      _ => panic!("Cannot convert {:?} to Option<bool>.", val),
    }
  }
}

impl Value {
  pub fn is_bool(&self) -> bool {
    self.value_type == ValueType::Bool
  }

  pub fn is_nil(&self) -> bool {
    self.value_type == ValueType::Nil
  }

  pub fn is_number(&self) -> bool {
    self.value_type == ValueType::Number
  }
}

impl Value {
  pub fn as_bool(&self) -> bool {
    unsafe { self.val_union.boolean }
  }

  pub fn as_number(&self) -> f64 {
    unsafe { self.val_union.number }
  }
}

impl Value {
  pub fn bool_val(value: bool) -> Self {
    Self {
      value_type: ValueType::Bool,
      val_union: ValUnion { boolean: value },
    }
  }

  pub fn nil_val() -> Self {
    Self {
      value_type: ValueType::Nil,
      val_union: ValUnion { number: 0.0 },
    }
  }

  pub fn number_val(value: f64) -> Self {
    Self {
      value_type: ValueType::Number,
      val_union: ValUnion { number: value },
    }
  }
}

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
