//! Manages the values of different identifiers

use std::collections::HashMap;

pub mod object;
pub use object::{Env, Object, FALSE, NOOP, NULL, TRUE};

use crate::evaluator::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Root {
    store: HashMap<String, Object>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nested {
    store: HashMap<String, Object>,
    outer: Box<dyn Env>,
}

impl Root {
    fn new() -> Self {
        Root {
            store: HashMap::new(),
        }
    }
}

impl Env for Root {
    // QUESTION: Is there a way to make the getter and setter work with both
    //           String and &str?
    fn get(&self, id: String) -> Result<Object> {
        if let Some(obj) = self.store.get(&id) {
            // QUESTION: Is there a better type to store so I'm not cloning?
            //           Box?
            Ok(obj.clone())
        } else {
            Err(Error::IdNotFound(id))
        }
    }

    fn set(&mut self, id: String, obj: Object) {
        self.store.insert(id, obj);
    }
}

impl Nested {
    fn new(outer: Box<dyn Env>) -> Self {
        Nested {
            store: HashMap::new(),
            outer,
        }
    }
}

impl Env for Nested {
    fn get(&self, id: String) -> Result<Object> {
        if let Some(obj) = self.store.get(&id) {
            Ok(obj.clone())
        } else {
            self.outer.get(id)
        }
    }

    fn set(&mut self, id: String, obj: Object) {
        self.store.insert(id, obj);
    }
}
