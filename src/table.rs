//! # Table
//!
//! A module which wraps [`std::collections::HashMap`].
//!
//! Currently, only support `{ObjString: Value}` pairs.

use std::{collections::HashMap, hash::Hash};

use crate::{object::ObjString, utils::Init, value::Value};

impl Hash for ObjString {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.hash.hash(state);
  }
}

#[derive(Debug, Clone, Default)]
pub struct Table(HashMap<ObjString, Value>);

impl Table {
  pub fn get(&self, key: &ObjString) -> Option<&Value> {
    self.0.get(key)
  }

  pub fn get_mut(&mut self, key: &ObjString) -> Option<&mut Value> {
    self.0.get_mut(key)
  }

  pub fn set(&mut self, key: ObjString, value: Value) -> Option<Value> {
    self.0.insert(key, value)
  }

  pub fn remove(&mut self, key: &ObjString) -> Option<Value> {
    self.0.remove(key)
  }

  pub fn free(&mut self) {
    self.0.clear()
  }
}

impl Init for Table {}
