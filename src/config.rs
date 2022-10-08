use std::collections::HashMap;
use crate::Group;

/// # Config struct
/// The ``Config`` struct provides access to the parsed configuration file
#[derive(Debug, Clone)]
pub struct Config {
    pub map: HashMap<String, Group>,
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
