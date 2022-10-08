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
}
