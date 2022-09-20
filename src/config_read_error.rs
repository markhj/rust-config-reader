/// # Config read errors
///
/// List of errors which can be returned when reading the
/// configuration file
#[derive(Debug)]
pub enum ConfigReadError {
    FileNotFound,
}
