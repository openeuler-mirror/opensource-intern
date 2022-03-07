//! A simple error handle, can output error type and info

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
/// A synthesis of all possible errors.
pub enum DagError {
    /// IO Error, like file not exist, etc.
    /// Here we simplefy it to a message(String).
    IOError(String),
    /// YAML Error, like format error, etc.
    YamlError(YamlError),
    /// Error that occurs when running dagrs.
    InnerError(InnerError),
}

#[derive(Debug, PartialEq, Eq)]
/// Format Error, point out which part has what kinds of error.
pub enum FormatError {
    /// Not start with 'dagrs'.
    StartWordError,
    /// A task have no name filed, `String` points out task's id.
    NoName(String),
    /// Run field format error
    RunScriptError(String)
}

#[derive(Debug, PartialEq, Eq)]
/// Error that occurs when parsing YAML file.
pub enum YamlError {
    /// Yaml Parser defined error type.
    YamlParserError(yaml_rust::ScanError),
    /// Format Error type.
    YamlFormatError(FormatError),
}

#[derive(Debug, PartialEq, Eq)]
/// Error that occurs when running dagrs
pub enum InnerError {
    /// Dependency task not exist
    RelyTaskIllegal(String),
}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartWordError => write!(f, "YAML file not start with 'dagrs:'"),
            Self::NoName(id) => write!(f, "Task[ID:{}] name not found", id),
            Self::RunScriptError(id) => write!(f, "Task[ID:{}] run script format error", id),
        }
    }
}

impl Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::YamlFormatError(e) => e.fmt(f),
            Self::YamlParserError(e) => e.fmt(f),
        }
    }
}

impl Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RelyTaskIllegal(name) => write!(f, "Task[Name:{}] rely tasks not exist!", name),
        }
    }
}

impl Display for DagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::IOError(e) => e.fmt(f),
            Self::YamlError(e) => e.fmt(f),
            Self::InnerError(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for DagError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e.to_string())
    }
}

impl From<yaml_rust::ScanError> for DagError {
    fn from(e: yaml_rust::ScanError) -> Self {
        Self::YamlError(YamlError::YamlParserError(e))
    }
}

impl DagError {
    /// Throw a format error
    ///
    /// # Example
    /// ```
    /// DagError::format_error(FormatError::NoName("a"));
    /// ```
    /// This will throw a error that says, task 'a' has no name field.
    pub fn format_error(error: FormatError) -> Self {
        Self::YamlError(YamlError::YamlFormatError(error))
    }

    /// Throw a inner error
    ///
    /// # Example
    /// ```
    /// DagError::inner_error(InnerError::RelyTaskIllegal("task 1"))
    /// ```
    /// This will throw a error that says, task with name "task 1" has non-exist rely tasks.
    pub fn inner_error(error: InnerError) -> Self {
        Self::InnerError(error)
    }
}
