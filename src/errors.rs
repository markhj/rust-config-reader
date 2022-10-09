/// # Config read errors
///
/// List of errors which can be returned when reading the configuration file
#[derive(Debug, PartialEq)]
pub enum ConfigReadError {
    /// File not found
    ///
    /// ``Err`` type returned when the specified configuration file doesn't exist,
    /// or isn't found where expected
    FileNotFound,

    /// Invalid syntax on line
    ///
    /// One or more lines in the configuration file has an invalid syntax, i.e.
    /// it's not a comment, not a group title or a configuration item
    InvalidSyntaxOnLine,
}

#[derive(Debug)]
pub enum ConfigValueError {
    InvalidBoolValue,
}