mod errors;
mod config;
mod group;

use std::{
    fs::File,
    io::{
        BufReader,
        BufRead,
    },
    path::Path,
    collections::HashMap,
};

use regex::Regex;

pub use crate::{
    config::Config,
    errors::{
        *,
        ConfigReadError::*,
    }
};

pub use crate::group::{
    ConfigurationItem,
    Group,
};

use crate::StringStrictness::*;
use crate::StringStrictnessBehavior::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StringStrictness {
    // ``Loose`` strictness allows strings with white-spaces in the configuration file
    Loose,

    // ``Forgivable`` string strictness allows strings without white-spaces
    // but strings with whitespaces must be encapsulated in quotes
    Forgivable,

    // Any string, also those with a single word, must be encapsulated in quotes
    Very,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringStrictnessBehavior {
    /// Do a ``panic!`` when an invalid string value is encountered in the configuration file
    Panic,

    /// Ignore/skip string values of invalid syntax
    Ignore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    pub string_strictness: StringStrictness,
    pub string_strictness_behavior: StringStrictnessBehavior,
}

/// # Get default ``Options`` struct
/// Retrieve all the default options for the ``Config``
pub fn get_default_options() -> Options {
    Options {
        string_strictness: Loose,
        string_strictness_behavior: Ignore,
    }
}

pub struct ConfigReader;

impl ConfigReader {
    /// # Read configuration file
    /// This function will attempt to load the specified
    /// configuration file as given in **filename**.
    ///
    /// If the file doesn't exist ``ConfigReadError::FileNotFound`` will be
    /// returned in the result.
    pub fn read(filename: &str, options: Option<Options>) -> Result<Config, ConfigReadError> {
        let opts: Options = options.unwrap_or(get_default_options());

        let path: &Path = Path::new(filename);
        if !path.exists() {
            return Err(FileNotFound);
        }

        let reader = BufReader::new(
            File::open(path).expect("Cannot open config file")
        );

        parse_config_file(reader, &opts)
    }
}

/// # Line syntax regular expressions
/// Struct containing the different regular expressions used to validate and
/// tokenize lines in the configuration file
struct LineSyntaxRegex {
    pub comment: Regex,
    pub group: Regex,
    pub item_any: Regex,
    pub empty_line: Regex,
    pub has_quotes: Regex,
    pub has_whitespace: Regex,
    pub non_string_type: Regex,
}

/// # Get ``LineSyntaxRegex``
/// Retrieve the ``LineSyntaxRegex`` struct with default regular expressions
fn get_line_syntax_regex() -> LineSyntaxRegex {
    LineSyntaxRegex {
        comment: Regex::new(r"^#").unwrap(),
        group: Regex::new(r"^\[([a-z][a-z_]*)\]$").unwrap(),
        item_any: Regex::new(r"^([a-z][a-z_]*)\s?=\s?(.*?)$").unwrap(),
        empty_line: Regex::new(r"^\s*$").unwrap(),
        has_quotes: Regex::new(r#"^"(.*?)"$"#).unwrap(),
        has_whitespace: Regex::new(r"\s+").unwrap(),
        non_string_type: Regex::new(r"^([0-9]+|true|false|yes|no|on|off)$").unwrap(),
    }
}

/// # Parse config file
/// If a file is successfully loaded, we will parse an instance of the ***Config***
/// struct, which consists of the HashMap under the hood.
///
/// The first layer to find all groups marked with ``[group]`` in the config file.
/// The second layer is splitting the lines with ``=``.
fn parse_config_file(
    file: BufReader<File>,
    options: &Options,
) -> Result<Config, ConfigReadError> {
    let regex: LineSyntaxRegex = get_line_syntax_regex();
    let mut config: Config = Config {
        map: HashMap::new(),
        cursor: None,
    };

    for line in file.lines() {
        let ln: String = line.unwrap().trim().to_string();

        if !is_line_valid(&ln, &regex) {
            return Err(InvalidSyntaxOnLine);
        }

        if regex.group.is_match(&ln) {
            config.add_group(regex.group.replace(&ln, "$1").to_string());
        }

        if regex.item_any.is_match(&ln) {
            match parse_line(&ln, &regex, &options) {
                Some(e) => {
                    config.insert(ConfigurationItem{
                        key: e[0].clone(),
                        value: e[1].clone(),
                    });
                },
                _ => {
                    if options.string_strictness_behavior == Panic {
                        panic!("Invalid syntax on configuration line: {}", ln);
                    }
                },
            }
        }
    }

    Ok(config)
}

/// # Parse line
/// Returns ``None`` if the line doesn't satisfy the requirements specified by the
/// ``string_strictness`` option. If the line is valid, ``key`` and ``value`` are returned.
/// If quotes are present on the string, they will be stripped.
fn parse_line(
    line: &String,
    regex: &LineSyntaxRegex,
    options: &Options,
) -> Option<[String; 2]> {
    let key: String = regex.item_any.replace(line.as_str(), "$1").trim().to_string();
    let mut val: String = regex.item_any.replace(line.as_str(), "$2").trim().to_string();

    let has_quotes = regex.has_quotes.is_match(val.as_str());
    let has_whitespace = regex.has_whitespace.is_match(val.as_str());
    let is_string = !regex.non_string_type.is_match(val.as_str());

    if is_string && !has_quotes && options.string_strictness == Very {
        return None;
    }

    if is_string && !has_quotes && has_whitespace && options.string_strictness == Forgivable {
        return None;
    }

    if has_quotes {
        val = regex.has_quotes.replace(val.as_str(), "$1").to_string();
    }

    Some([key, val])
}

/// # Is line valid
/// Returns true, if the line is of a valid syntax
fn is_line_valid(
    line: &String,
    regex: &LineSyntaxRegex,
) -> bool {
    regex.comment.is_match(line)
        || regex.group.is_match(line)
        || regex.empty_line.is_match(line)
        || regex.item_any.is_match(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_file_syntax() {
        assert_eq!(
            Err(InvalidSyntaxOnLine),
            ConfigReader::read("./test/test-config-invalid-syntax.txt", None)
        );
    }

    #[test]
    fn string_strictness_loose() {
        // The default configuration is ``Loose``
        let cfg: Config =  ConfigReader::read("./test/test-config.txt", None).unwrap();

        assert!(cfg.group("strict").unwrap().get("one_word").is_some());
        assert_eq!("Hello world", cfg.group("strict").unwrap().get("in_quotes").unwrap().value);
        assert_eq!("Hello world", cfg.group("strict").unwrap().get("no_quotes").unwrap().value);
    }

    #[test]
    fn string_strictness_forgivable() {
        let mut opts: Options = get_default_options();
        opts.string_strictness = Forgivable;

        let cfg: Config =  ConfigReader::read("./test/test-config.txt", Some(opts)).unwrap();

        assert!(cfg.group("strict").unwrap().get("one_word").is_some());
        assert_eq!("Hello world", cfg.group("strict").unwrap().get("in_quotes").unwrap().value);
        assert!(cfg.group("strict").unwrap().get("no_quotes").is_none());
    }

    #[test]
    fn string_strictness_strict() {
        let mut opts: Options = get_default_options();
        opts.string_strictness = Very;

        let cfg: Config = ConfigReader::read("./test/test-config.txt", Some(opts)).unwrap();

        assert!(cfg.group("strict").unwrap().get("one_word").is_none());
        assert_eq!("Hello world", cfg.group("strict").unwrap().get("in_quotes").unwrap().value);
        assert!(cfg.group("strict").unwrap().get("no_quotes").is_none());
    }

    #[test]
    fn file_not_found() {
        assert!(ConfigReader::read("doesnt-exist.txt", None).is_err());
    }

    #[test]
    fn get() {
        let cfg: Config = ConfigReader::read("./test/test-config.txt", None).unwrap();
        assert_eq!("value", cfg.group("group").unwrap().get("property").unwrap().value);
        assert_eq!("25", cfg.group("group").unwrap().get("underscore_value").unwrap().value);
        assert_eq!("Hello world", cfg.group("another").unwrap().get("hello").unwrap().value);

        // Check that None is returned when key doesn't exist
        assert!(cfg.group("another").unwrap().get("nope").is_none());
    }

    #[test]
    fn group_not_existing() {
        assert!(ConfigReader::read("./test/test-config.txt", None).unwrap().group("nope").is_none());
    }

    #[test]
    fn get_or() {
        let cfg: Config = ConfigReader::read("./test/test-config.txt", None).unwrap();
        assert_eq!("fallback", cfg.group("group").unwrap().get_or("nope", "fallback"))
    }

    #[test]
    fn has_group() {
        let cfg: Config = ConfigReader::read("./test/test-config.txt", None).unwrap();
        assert!(cfg.has_group("group"));
        assert!(!cfg.has_group("non_existing"));
    }

    #[test]
    fn group_has_key() {
        let cfg: Config = ConfigReader::read("./test/test-config.txt", None).unwrap();
        assert!(cfg.group("group").unwrap().has("name"));
        assert!(!cfg.group("group").unwrap().has("nope"));
    }

    #[test]
    fn keys() {
        let cfg: Config = ConfigReader::read("./test/test-config.txt", None).unwrap();
        let keys: Vec<String> = cfg.group("group").unwrap().keys();

        assert!(keys.contains(&"property".to_string()));
        assert!(keys.contains(&"underscore_value".to_string()));
        assert!(keys.contains(&"name".to_string()));
        assert_eq!(3, keys.len());
    }

    #[test]
    fn for_each_group() {
        ConfigReader::read("./test/test-config.txt", None)
            .unwrap()
            .for_each_group(|_key: &str, _group: &Group| {

            });
    }

    #[test]
    fn for_each() {
        ConfigReader::read("./test/test-config.txt", None)
            .unwrap()
            .group("group")
            .unwrap()
            .for_each(|_cfg_item: &ConfigurationItem| {

            });
    }

    #[test]
    fn typecasting() {
        let cfg = ConfigReader::read("./test/test-config.txt", None).unwrap();
        let item = cfg.group("group").unwrap().get("underscore_value").unwrap();

        assert_eq!(String::from("25"), item.get());
        assert_eq!(25i32, item.as_i32());
        assert_eq!(25u32, item.as_u32());
        assert_eq!(25f32, item.as_f32());
        assert_eq!(25i64, item.as_i64());
        assert_eq!(25u64, item.as_u64());
        assert_eq!(25f64, item.as_f64());
        assert_eq!("25", item.as_str());
    }

    #[test]
    fn bools() {
        let cfg = ConfigReader::read("./test/test-config.txt", None).unwrap();
        let bools = cfg.group("bools").unwrap();

        let true_vals: [&str; 4] = ["one", "true", "on", "yes"];
        for x in true_vals {
            assert!(bools.get(x).unwrap().as_bool().unwrap());
        }

        let false_vals: [&str; 4] = ["zero", "false", "off", "no"];
        for x in false_vals {
            assert!(!bools.get(x).unwrap().as_bool().unwrap());
        }

        assert!(bools.get("invalid").unwrap().as_bool().is_err());
    }
}
