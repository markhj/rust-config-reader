use std::collections::HashMap;
use crate::{ConfigurationItem, Group};

/// # Config struct
/// The ``Config`` struct provides access to the parsed configuration file
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub map: HashMap<String, Group>,
    pub cursor: Option<String>,
}

impl Config {
    /// # Group
    /// Access a specific group by ``group``.
    /// ``None`` option is returned when the group doesn't exist.
    pub fn group(&self, group: &str) -> Option<Group> {
        match self.has_group(group) {
            true => Some(self.map.get(group).unwrap().clone()),
            _ => None,
        }
    }

    /// # Insert
    /// Insert ConfigurationItem into the latest added group
    pub fn insert(&mut self, val: ConfigurationItem) {
        let group: &String = self.cursor.as_ref().unwrap();
        if self.has_group(&group) {
            let mut grp = self.map.get(group.as_str()).unwrap().clone();
            grp.pairs.insert(val.key.to_string(), val);
            self.map.insert(group.clone(), grp.clone());
        }
    }

    /// # Add group
    /// Insert a new group into the ``Config`` instance.
    /// This is mostly used by the internal parser functions, but can also be
    /// used in real-life use-cases.
    pub fn add_group(&mut self, name: String) {
        self.cursor = Some(name.clone());
        self.map.insert(name.to_string(), Group {
            pairs: HashMap::new(),
        });
    }

    /// # For each group
    /// Loop over every group in the configuration
    pub fn for_each_group<F: Fn(&str, &Group)>(&self, closure: F) {
        for group in &self.map {
            closure(group.0, group.1);
        }
    }

    /// # Groups
    /// Returns a ``Vec<String>`` collection containing the groups in the config file
    pub fn groups(&self) -> Vec<String> {
        Vec::from_iter(self.map.keys().map(|e: &String| e.to_string()))
    }

    /// # has_group
    /// Returns true if the group exists in the config file
    /// The ``group`` parameters is case-sensitive
    pub fn has_group(&self, group: &str) -> bool {
        self.map.contains_key(&group.to_string())
    }
}
