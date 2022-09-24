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
use crate::config_read_error::{*, ConfigReadError::*};
use crate::config::Config;

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
    fn read_file() {
        let categories = read("./test/test-config.txt").unwrap();

        assert_eq!("Group", categories.get("group", "name").unwrap());
        assert_eq!("Another", categories.get("another", "name").unwrap());
        assert_eq!("Hello world", categories.get("another", "hello").unwrap());
        assert_eq!("25", categories.get("group", "underscore_value").unwrap());
        assert!(categories.get("exists", "not").is_err());
    }

    #[test]
    fn file_not_found() {
        assert!(read("doesnt-exist.txt").is_err());
    }
}
