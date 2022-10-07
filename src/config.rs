use std::collections::HashMap;
use crate::ConfigReadError::{
    self,
    GroupNotFound,
    KeyInGroupNotFound,
};

/// # Config struct
/// The Configuration struct which implements the `´get`` and ``get_or``
/// functions, and the HashMap of groups and subsequent key/value pairs
#[derive(Debug, Clone)]
pub struct Config {
    pub map: HashMap<String, HashMap<String, String>>,
}

impl Config {
    /// # Get (configuration item)
    /// The ``group`` parameter refers to headlines, like ``[headline]``, in the config file.
    /// The ``key`` is the first part of a line, such as ``key = value``
    ///
    /// This function returns ``GroupNotFound`` as result error, when the group doesn't exist.
    /// It returns ``KeyInGroupNotFound`` when a key doesn't exist in a group
    pub fn get(&self, group: &str, key: &str) -> Result<String, ConfigReadError> {
        if !self.has_group(group) {
            return Err(GroupNotFound);
        }
        let properties: &HashMap<String, String> = self.map.get(group).unwrap();
        if !properties.contains_key(&key.to_string()) {
            return Err(KeyInGroupNotFound);
        }
        Ok(properties.get(&key.to_string()).unwrap().clone())
    }

    /// # Get (or return fallback)
    /// If you want to provide a default/fallback value in case the key doesn't
    /// exist in the config file. Since this method supports a fallback, we don't
    /// need to return the Result type
    pub fn get_or(&self, group: &str, key: &str, fallback: &str) -> String {
        self.get(group, key).unwrap_or(fallback.to_string())
    }

    /// # Groups
    /// Returns a ``Vec<String>`` collection containing the groups in the config file
    pub fn groups(&self) -> Vec<String> {
        Vec::from_iter(self.map.keys().map(|e: &String| e.to_string()))
    }

    /// # Keys
    /// Returns a ``Vec<String>`` collection of keys found in a specified group
    pub fn keys(&self, group: &str) -> Vec<String> {
        Vec::from_iter(self.map.get(group).unwrap().keys().map(|e: &String| e.to_string()))
    }

    /// # has_group
    /// Returns true if the group exists in the config file
    /// The ``group`` parameters is case-sensitive
    pub fn has_group(&self, group: &str) -> bool {
        self.map.contains_key(&group.to_string())
    }
}
