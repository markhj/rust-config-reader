use std::collections::HashMap;

/// # Config struct
#[derive(Debug)]
pub struct Config {
    pub map: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn get(&self, group : &str, key : &str) -> Result<String, ()> {
        if !self.map.contains_key(&group.to_string()) {
            return Err(());
        }
        let properties : &HashMap<String, String> = self.map.get(group).unwrap();
        if !properties.contains_key(&key.to_string()) {
            return Err(());
        }
        return Ok(properties.get(&key.to_string()).unwrap().clone());
    }
}
