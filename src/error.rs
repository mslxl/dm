use std::fmt::Write;

use crate::cfg::transaction::Transaction;

#[derive(Debug)]
pub enum ErrorLevel {
    Warnning,
    Error,
}

pub struct Error {
    message: String,
    level: ErrorLevel,
    quickfix: Option<QuickFix>,
}

impl Error {
    pub fn err(msg: String) -> Self {
        Self {
            message: msg,
            level: ErrorLevel::Error,
            quickfix: None,
        }
    }
    pub fn warn(msg: String) -> Self {
        Self {
            message: msg,
            level: ErrorLevel::Warnning,
            quickfix: None,
        }
    }
    pub fn suggest(mut self, suggest: String) -> Self {
        self.quickfix = Some(QuickFix {
            suggestion: suggest,
            action: None,
        });
        self
    }
    pub fn suggest_with_action<T>(mut self, suggest: String, action: T) -> Self
    where
        T: FnOnce(&mut Transaction) -> Result<(), QuickFixError> + 'static,
    {
        self.quickfix = Some(QuickFix {
            suggestion: suggest,
            action: Some(Box::new(action)),
        });
        self
    }
}


impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("message", &self.message)
            .field("level", &self.level)
            .field("quickfix", &self.quickfix)
            .finish()
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level {
            ErrorLevel::Error => f.write_str("Error: "),
            ErrorLevel::Warnning => f.write_str("Warnning: "),
        }?;

        f.write_str(&self.message)?;

        if let Some(quickfix) = &self.quickfix {
            f.write_fmt(format_args!("\n{}\n", &quickfix.suggestion))
        } else {
            f.write_char('\n')
        }
    }
}

pub struct QuickFix {
    pub suggestion: String,
    pub action: Option<Box<dyn FnOnce(&mut Transaction) -> Result<(), QuickFixError>>>,
}

impl std::fmt::Debug for QuickFix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuickFix")
            .field("suggestion", &self.suggestion)
            .finish()
    }
}

pub struct QuickFixError(String);

impl std::fmt::Debug for QuickFixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("QuickFixError").field(&self.0).finish()
    }
}

impl std::fmt::Display for QuickFixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("QuickFixError: {}", self.0))
    }
}

impl std::error::Error for QuickFixError {}
