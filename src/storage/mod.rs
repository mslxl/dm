pub mod file;

use std::fmt::Debug;

pub struct StorageError {
    message: String,
}

impl StorageError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl <T> From<T> for StorageError where T:ToString {
    fn from(err: T) -> Self {
      Self::new(err.to_string())
    }
}

impl Debug for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

