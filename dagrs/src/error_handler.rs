//! A simple error handle, can output error type and info

use std::fmt::Display;


#[derive(Debug, PartialEq, Eq)]
/// Format Error may occur in yaml file
pub enum FormatErrorMark {
    /// Not start with 'dagrs'
    StartWordError,
    /// A task have no name filed
    NoName,
    /// The task ID dependent on the task does not exist
    RelyIDIllegal,
}

#[derive(Debug, PartialEq, Eq)]
/// Format Error, point out which part has what error type.
pub struct FormatError {
    /// which task definition has errors.
    id: String,
    /// Has what kinds of errors.
    mark: FormatErrorMark,
}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.mark {
            FormatErrorMark::StartWordError => write!(f, "Not start with 'dagrs:'"),
            FormatErrorMark::NoName => write!(f, "Task[ID:{}] name not found", self.id),
            FormatErrorMark::RelyIDIllegal => write!(f, "Task[ID:{}] rely ID not found", self.id),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
/// A synthesis of all possible errors.
pub enum DagError {
    /// IO Error, like file not exist, etc.
    /// Here we simplefy it to a message(String).
    IOError(String),
    /// Yaml Parser defined error type.
    YamlParserError(yaml_rust::ScanError),
    /// Format Error type.
    YamlFormatError(FormatError),
}

impl Display for DagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::IOError(e) => e.fmt(f),
            Self::YamlParserError(e) => e.fmt(f),
            Self::YamlFormatError(e) => e.fmt(f),
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
        Self::YamlParserError(e)
    }
}

impl DagError {
    /// Throw a format error
    /// 
    /// # Example
    /// ```
    /// DagError::format_error("a", FormatErrorMark::NoName);
    /// ```
    /// This will throw a error that says, task 'a' has no name field.
    pub fn format_error(id: &str, mark: FormatErrorMark) -> Self {
        Self::YamlFormatError(FormatError {
            id: id.into(),
            mark,
        })
    }
}
