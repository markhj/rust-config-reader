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

pub use crate::config_read_error::{*, ConfigReadError::*};
pub use crate::config::Config;

mod config_read_error;
mod config;

/// # Read configuration file
/// This function will attempt to load the specified
/// configuration file as given in **filename**.
///
/// If the file doesn't exist ***ConfigReadError*** will be
/// returned in the result.
pub fn read(filename : &str) -> Result<Config, ConfigReadError> {
    let path : &Path = Path::new(filename);

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
fn parse_config_file(file : BufReader<File>) -> Config {
    let regex_config = Regex::new(r"^[a-z][a-z_]+\s?=\s?.*?$").unwrap();
    let regex_group = Regex::new(r"^\[([a-z][a-z_]*)\]$").unwrap();

    let mut cfg : HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut grp : Option<String> = None;
    let mut tmp : HashMap<String, String> = HashMap::new();

    for line in file.lines() {
        let ln : String = line.unwrap().trim().to_string();

        // If we hold an existing group of key/value pairs, when we encounter
        // a new group, we insert the content into the HashMap
        if regex_group.is_match(&ln) && grp.is_some() {
            cfg.insert(grp.clone().unwrap(), tmp.clone());
        }

        if regex_group.is_match(&ln) {
            grp = Some(regex_group.replace_all(&ln, "$1").to_string());
            tmp = HashMap::new();
            continue;
        }

        if grp.is_none() || !regex_config.is_match(&ln) {
            continue;
        }

        let v : Vec<&str> = ln.split("=").collect();

        tmp.insert(v[0].trim().to_string(), v[1].trim().to_string());
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
    fn keys() {
        let categories = read("./test/test-config.txt").unwrap();
        let keys: Vec<String> = categories.keys("group");

        assert!(keys.contains(&"property".to_string()));
        assert!(keys.contains(&"underscore_value".to_string()));
        assert!(keys.contains(&"name".to_string()));
        assert_eq!(3, keys.len());
    }

    #[test]
    fn groups() {
        let categories = read("./test/test-config.txt").unwrap();
        let keys: Vec<String> = categories.groups();

        assert!(keys.contains(&"group".to_string()));
        assert!(keys.contains(&"another".to_string()));
        assert_eq!(2, keys.len());
    }

    #[test]
    fn group_not_found() {
        assert_eq!(
            GroupNotFound,
            read("./test/test-config.txt").unwrap().get("fake_group", "hello").err().unwrap()
        );
    }

    #[test]
    fn key_in_group_not_found() {
        assert_eq!(
            KeyInGroupNotFound,
            read("./test/test-config.txt").unwrap().get("group", "hello").err().unwrap()
        );
    }

    #[test]
    fn read_file() {
        let categories = read("./test/test-config.txt").unwrap();

        assert_eq!("Group", categories.get("group", "name").unwrap());
        assert_eq!("Another", categories.get("another", "name").unwrap());
        assert_eq!("Hello world", categories.get("another", "hello").unwrap());

        // Check the underscore character
        assert_eq!("25", categories.get("group", "underscore_value").unwrap());

        // Non-existing group (as Result error)
        assert!(categories.get("exists", "not").is_err());

        // Non-existing key in existing group (as Result error)
        assert!(categories.get("group", "not").is_err());

        // Check default works when group doesn't exist
        assert_eq!("Default", categories.get_or("nogroup", "whatever", "Default"));

        // Check default when a key doesn't exist in a group
        assert_eq!("Default", categories.get_or("group", "nokey", "Default"));
    }

    #[test]
    fn file_not_found() {
        assert!(read("doesnt-exist.txt").is_err());
    }
}
