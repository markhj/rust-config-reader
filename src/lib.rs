mod config_read_error;
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
    config_read_error::{
        *,
        ConfigReadError::*,
    }
};

pub use crate::group::{
    ConfigurationItem,
    Group,
};

/// # Read configuration file
/// This function will attempt to load the specified
/// configuration file as given in **filename**.
///
/// If the file doesn't exist ``ConfigReadError::FileNotFound`` will be
/// returned in the result.
pub fn read(filename: &str) -> Result<Config, ConfigReadError> {
    let path: &Path = Path::new(filename);
    if !path.exists() {
        return Err(FileNotFound);
    }

    let reader = BufReader::new(
        File::open(path).expect("Cannot open config file")
    );

    Ok(parse_config_file(reader))
}

/// # Parse config file
/// If a file is successfully loaded, we will parse an instance of the ***Config***
/// struct, which consists of the HashMap under the hood.
///
/// The first layer to find all groups marked with ``[group]`` in the config file.
/// The second layer is splitting the lines with ``=``.
fn parse_config_file(file: BufReader<File>) -> Config {
    let regex_config: Regex = Regex::new(r"^[a-z][a-z_]+\s?=\s?.*?$").unwrap();
    let regex_group: Regex = Regex::new(r"^\[([a-z][a-z_]*)\]$").unwrap();

    let mut cfg: HashMap<String, Group> = HashMap::new();
    let mut grp: Option<String> = None;
    let mut tmp: Group = Group::new();

    for line in file.lines() {
        let ln : String = line.unwrap().trim().to_string();

        // If we hold an existing group of key/value pairs, when we encounter
        // a new group, we insert the content into the HashMap
        if regex_group.is_match(&ln) && grp.is_some() {
            cfg.insert(grp.clone().unwrap(), tmp.clone());
        }

        if regex_group.is_match(&ln) {
            grp = Some(regex_group.replace_all(&ln, "$1").to_string());
            tmp = Group::new();
            continue;
        } else if grp.is_none() || !regex_config.is_match(&ln) {
            continue;
        }

        let pair: Vec<&str> = ln.split("=").collect();

        let key: String = pair[0].trim().to_string();
        let item: ConfigurationItem = ConfigurationItem::new(
            key.clone(),
            pair[1].trim().to_string(),
        );
        tmp.pairs.insert(key, item);
    }

    // Insert the final (temp.) list of key/value pairs
    cfg.insert(grp.clone().unwrap(), tmp.clone());

    Config {
        map: cfg,
    }
}

/// # Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_not_found() {
        assert!(read("doesnt-exist.txt").is_err());
    }

    #[test]
    fn get() {
        let cfg: Config = read("./test/test-config.txt").unwrap();
        assert_eq!("value", cfg.group("group").unwrap().get("property").unwrap().value);
        assert_eq!("25", cfg.group("group").unwrap().get("underscore_value").unwrap().value);
        assert_eq!("Hello world", cfg.group("another").unwrap().get("hello").unwrap().value);

        // Check that None is returned when key doesn't exist
        assert!(cfg.group("another").unwrap().get("nope").is_none());
    }

    #[test]
    fn group_not_existing() {
        assert!(read("./test/test-config.txt").unwrap().group("nope").is_none());
    }

    #[test]
    fn get_or() {
        let cfg: Config = read("./test/test-config.txt").unwrap();
        assert_eq!("fallback", cfg.group("group").unwrap().get_or("nope", "fallback"))
    }

    #[test]
    fn has_group() {
        let cfg: Config = read("./test/test-config.txt").unwrap();
        assert!(cfg.has_group("group"));
        assert!(!cfg.has_group("non_existing"));
    }

    #[test]
    fn group_has_key() {
        let cfg: Config = read("./test/test-config.txt").unwrap();
        assert!(cfg.group("group").unwrap().has("name"));
        assert!(!cfg.group("group").unwrap().has("nope"));
    }

    #[test]
    fn keys() {
        let cfg: Config = read("./test/test-config.txt").unwrap();
        let keys: Vec<String> = cfg.group("group").unwrap().keys();

        assert!(keys.contains(&"property".to_string()));
        assert!(keys.contains(&"underscore_value".to_string()));
        assert!(keys.contains(&"name".to_string()));
        assert_eq!(3, keys.len());
    }

    #[test]
    fn for_each_group() {
        read("./test/test-config.txt")
            .unwrap()
            .for_each_group(|_key: &str, _group: &Group| {

            });
    }

    #[test]
    fn for_each() {
        read("./test/test-config.txt")
            .unwrap()
            .group("group")
            .unwrap()
            .for_each(|_cfg_item: &ConfigurationItem| {

            });
    }
}
