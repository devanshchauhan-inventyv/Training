// employee_processing.rs
use crate::common::{Employee, Position};
use crate::employee_task::file_operations::{read_data, write_data};
use crate::hashmap::employee_task::file_operations::vec_to_hashmap;
use std::fs;
use crate::employee_hashmap::file_operations;
/// Processes employee data based on their positions and skills.
///
/// Reads employee data from a file specified by `file_path`, filters employees
/// based on their positions and skills, and then writes the filtered data
/// into separate JSON files for different positions and skills combinations.
/// 
pub fn process_employee_data(file_path: &str) {
    let final_data: Vec<Employee> = read_data(file_path);
    println!("{:?}",final_data);

    let mut mid_rust: Vec<Employee> = Vec::new();
    let mut jr_java: Vec<Employee> = Vec::new();
    let mut sr_c_hash: Vec<Employee> = Vec::new();
     //  Separate employees based on positions and skills
    for iter in final_data {
        if iter.position == Some(Position::SoftwareDeveloper)
            && iter.skills.contains(&String::from("Rust"))
        {
            mid_rust.push(iter);
        } else if iter.position == Some(Position::JrSoftwareDeveloper)
            && iter.skills.contains(&String::from("Java"))
        {
            jr_java.push(iter);
        } else if iter.position == Some(Position::SrSoftwareDeveloper)
            || iter.skills.contains(&String::from("C#"))
        {
            sr_c_hash.push(iter);
        }
    }
    // println!("{:#?}")
    // Converting Vec into Hashmap
    let mid_rust = vec_to_hashmap(mid_rust);
    let jr_java=vec_to_hashmap(jr_java);
    let sr_c_hash=vec_to_hashmap(sr_c_hash);

 //  Serialize filtered data to JSON strings
    let mid_rust =
        serde_json::to_string_pretty(&mid_rust).expect("Failed during serializing to JSON");
    let jr_java =
        serde_json::to_string_pretty(&jr_java).expect("Failed during serializing to JSON");
    let sr_c_hash =
        serde_json::to_string_pretty(&sr_c_hash).expect("Failed during serializing to JSON");
 //  Write JSON strings to separate files for specific position and skill combination
    fs::write("src/hashmap/employee_task/data/mid_rust.json", mid_rust).expect("Failed to write to file");
    fs::write("src/hashmap/employee_task/data/jr_java.json", jr_java).expect("Failed to write to file");
    fs::write("src/hashmap/employee_task/data/sr_c_hash.json", sr_c_hash).expect("Failed to write to file");
}
