/// # Config read errors
///
/// List of errors which can be returned when reading the
/// configuration file
#[derive(Debug, PartialEq)]
pub enum ConfigReadError {
    FileNotFound,
    GroupNotFound,
    KeyInGroupNotFound,
}
