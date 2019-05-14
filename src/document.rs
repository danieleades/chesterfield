use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Serialize, Deserialize)]
pub struct Document<T> {
    _id: String,
    _rev: String,

    pub data: T,
}

impl<T> Document<T> {
    pub fn new(data: T, id: impl Into<String>, rev: impl Into<String>) -> Self {
        Document {
            _id: id.into(),
            _rev: rev.into(),
            data,
        }
    }

    pub fn _id(&self) -> &str {
        &self._id
    }

    pub fn _rev(&self) -> &str {
        &self._rev
    }
}

impl<T> ops::Deref for Document<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> ops::DerefMut for Document<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
