use std::collections::HashMap;

/// # Configuration item
/// Struct containing information a single line in the configuration file
#[derive(Debug, Clone)]
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
}

/// # Group
/// Group struct containing one or more configuration items mapped
/// to a ``String`` key
#[derive(Debug, Clone)]
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
