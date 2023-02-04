use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum DMError {
    #[error(transparent)]
    #[diagnostic()]
    IOError(#[from] std::io::Error),

    #[error("LockError: {msg}")]
    #[diagnostic()]
    LockError {
        msg: String,
        #[help]
        advice: String,
    },

    #[error("ProfileError({kind:?}): {msg}")]
    ProfileError {
        kind: ProfileErrorKind,
        msg: String,
        #[help]
        advice: Option<String>,
    },
    #[error(transparent)]
    #[diagnostic()]
    TomlSerError(#[from] toml_edit::ser::Error),
    #[error(transparent)]
    #[diagnostic()]
    TomlDeError(#[from] toml_edit::de::Error),
}

#[derive(Debug)]
pub enum ProfileErrorKind {
    DuplicateCreate,
    NotExists
}
