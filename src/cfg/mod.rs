pub mod transcation;
pub mod global;
pub mod group;
pub mod file;

use std::fmt::Debug;

pub struct CfgError {
    message: String,
}

impl CfgError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Debug for CfgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
