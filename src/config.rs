use std::collections::HashMap;

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
    pub fn get(&self, group : &str, key : &str) -> Result<String, ()> {
        if !self.map.contains_key(&group.to_string()) {
            return Err(());
        }
        let properties : &HashMap<String, String> = self.map.get(group).unwrap();
        if !properties.contains_key(&key.to_string()) {
            return Err(());
        }
        Ok(properties.get(&key.to_string()).unwrap().clone())
    }

    /// # Get (or return fallback)
    /// If you want to provide a default/fallback value in case the key doesn't
    /// exist in the config file. Since this method supports a fallback, we don't
    /// need to return the Result type
    pub fn get_or(&self, group : &str, key : &str, fallback : &str) -> String {
        self.get(group, key).unwrap_or(fallback.to_string())
    }

    /// # Groups
    /// Returns the a ``Vec<String>`` of the groups in the config file
    pub fn groups(&self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();
        for x in &self.map {
            keys.insert(0, x.0.to_string());
        }
        keys
    }

    /// # Keys
    /// Returns the a ``Vec<String>`` of keys found in a specific group
    pub fn keys(&self, group: &str) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();
        for x in self.map.get(group).unwrap().keys() {
            keys.insert(0, x.to_string());
        }
        keys
    }
}
