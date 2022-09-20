use std::collections::HashMap;

/// # Config struct
#[derive(Debug)]
pub struct Config {
    pub map: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn get(&self, group : String, key : String) -> Result<String, ()> {
        if !self.map.contains_key(&group) {
            return Err(());
        }
        let properties : &HashMap<String, String> = self.map.get(&group).unwrap();
        if !properties.contains_key(&key) {
            return Err(());
        }
        return Ok(properties.get(&key).unwrap().clone());
    }
}
