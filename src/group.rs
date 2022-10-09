use std::collections::HashMap;
use crate::errors::ConfigValueError::{self, *};

/// # Configuration item
/// Struct containing information a single line in the configuration file
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationItem {
    pub key: String,
    pub value: String,
}

impl ConfigurationItem {
    /// # New ConfigurationItem
    /// Create an instance of the ConfigurationItem struct with ``key`` and
    /// ``value`` filled out
    pub fn new(key: String, value: String) -> ConfigurationItem {
        ConfigurationItem {
            key,
            value,
        }
    }

    pub fn get(&self) -> String { self.value.clone() }
    pub fn as_str(&self) -> &str { self.value.as_str() }
    pub fn as_i32(&self) -> i32 { self.value.parse::<i32>().unwrap() }
    pub fn as_u32(&self) -> u32 { self.value.parse::<u32>().unwrap() }
    pub fn as_f32(&self) -> f32 { self.value.parse::<f32>().unwrap() }
    pub fn as_i64(&self) -> i64 { self.value.parse::<i64>().unwrap() }
    pub fn as_u64(&self) -> u64 { self.value.parse::<u64>().unwrap() }
    pub fn as_f64(&self) -> f64 { self.value.parse::<f64>().unwrap() }

    /// # As bool
    /// Parse the value as a boolean.
    ///
    /// The following values are interpreted as true:
    /// 1, true, on, yes
    ///
    /// Similarly, these values are interpreted as false:
    /// 0, false, off, no
    ///
    /// This allows to create config lines like:
    /// ``key = true`` as well as ``key = 1``, both of which will be interpreted the same way
    pub fn as_bool(&self) -> Result<bool, ConfigValueError> {
        match self.value.as_str() {
            "1" => Ok(true),
            "true" => Ok(true),
            "yes" => Ok(true),
            "on" => Ok(true),
            "0" => Ok(false),
            "false" => Ok(false),
            "no" => Ok(false),
            "off" => Ok(false),
            _ => Err(InvalidBoolValue),
        }
    }

    /// # As bool (graceful)
    /// Similar to ``as_bool`` but doesn't require unwrapping.
    /// Instead, invalid syntax is interpreted as false
    pub fn as_bool_grf(&self) -> bool {
        self.as_bool().unwrap_or(false)
    }
}

/// # Group
/// Group struct containing one or more configuration items mapped
/// to a ``String`` key
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub pairs: HashMap<String, ConfigurationItem>,
}

impl Group {
    /// # New Group
    /// Create an empty ``Group`` struct
    pub fn new() -> Group {
        Group {
            pairs: HashMap::new(),
        }
    }

    /// # For each item in the group
    /// Iterate over the items in the group and handle each item in the list
    pub fn for_each<F: Fn(&ConfigurationItem)>(&self, closure: F) {
        for x in &self.pairs {
            closure(x.1);
        }
    }

    /// # Keys
    /// Returns a ``Vec<String>`` collection of keys found in the Group
    pub fn keys(&self) -> Vec<String> {
        Vec::from_iter(self.pairs.keys().map(|e: &String| e.to_string()))
    }

    /// # Has (key)
    /// Returns true, if the ``key`` is found in the Group
    pub fn has(&self, key: &str) -> bool {
        self.pairs.contains_key(key)
    }

    /// # Get (configuration item)
    /// Get the configuration item identified by the ``key``.
    /// If the key doesn't exist an ``None`` option will be returned
    pub fn get(&self, key: &str) -> Option<ConfigurationItem> {
        match self.has(key.clone()) {
            true => Some(self.pairs.get(key).unwrap().clone()),
            _ => None,
        }
    }

    /// # Get (or return fallback)
    /// If you want to provide a default/fallback value in case the key doesn't
    /// exist in the config file. Since this method supports a fallback, we don't
    /// need to return the Result type
    pub fn get_or(&self, key: &str, fallback: &str) -> String {
        self.get(key).unwrap_or(ConfigurationItem {
            key: key.to_string(),
            value: fallback.to_string(),
        }).value
    }
}
