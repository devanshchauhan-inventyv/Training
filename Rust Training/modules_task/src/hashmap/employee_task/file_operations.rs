use serde_json::{json, Value};

use crate::common::*;
use std::{collections::HashMap, fs};
pub fn read_data(file_path: &str) -> Vec<Employee> {
    let data = fs::read_to_string(file_path).expect("Failed to read");
    serde_json::from_str(&data).expect("Something went wrong")
}

pub fn vec_to_hashmap(data: Vec<Employee>) -> Vec<HashMap<String, Value>> {
    let mut final_data: Vec<HashMap<String, Value>> = Vec::new();

    for i in data {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("name".to_string(), Value::String(i.name));
        map.insert("age".to_string(), json!(i.age));
        map.insert("Skills".to_string(), json!(i.skills));
        if !i.position.is_none() {
            map.insert("position".to_string(), json!(i.position));
        }
        if !i.experiance_y.is_none() {
            map.insert("experiance(y)".to_string(), json!(i.experiance_y));
        }
        final_data.push(map);
    }

    final_data
}
