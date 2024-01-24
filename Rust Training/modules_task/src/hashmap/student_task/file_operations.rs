use std::{collections::HashMap, fs};

use serde_json::{json, Value};

use super::Student;

/// Reads student data from a JSON file and Returns a vector of `Student` structs read from the file.
///
pub fn read_data(file_path: &str) -> Vec<Student> {
    let data = fs::read_to_string(file_path).expect("Failed to read");
    serde_json::from_str(&data).expect("Something went wrong")
}

/// Writes student data to a JSON file.
///
pub fn write_data(file_path: &str, data: &Vec<HashMap<String,Value>>) {
    let updated_data = serde_json::to_string_pretty(data).expect("Failed to convert to JSON");
    fs::write(file_path, updated_data).expect("Something went wrong");
}

pub fn vec_to_hashmap(data : Vec<Student>) -> Vec<HashMap<String , Value>>{
    let mut final_data :Vec<HashMap<String , Value>> = Vec::new();

    for i in data{
        let mut map : HashMap<String , Value> = HashMap::new();
        map.insert( "name".to_string() , Value::String(i.name));
        map.insert( "phone".to_string() , json!(i.phone));
        map.insert( "email".to_string() , json!(i.email));
        map.insert( "city".to_string() , json!(i.city));
        map.insert( "address".to_string() , json!(i.address));
        map.insert( "marks".to_string() , json!(i.marks));
        if !i.percentage.is_none(){
            map.insert( "percentage".to_string() , json!(i.percentage));
        }
        if !i.grade.is_none(){
            map.insert( "grade".to_string() , json!(i.grade));
        }
        final_data.push(map);
    } 

    final_data
}