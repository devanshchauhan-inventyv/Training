use serde::{Deserialize,Serialize};
use std::fs;

#[derive(Debug,Deserialize,Serialize)]
struct Employee{
    name:String,
    age:u8,
    skills:Vec<String>,
    position:Option<Position>,
    #[serde(rename = "experiance(y)")]
    experiance_y:Option<u8>,

}
#[derive(Debug,Deserialize,Serialize,PartialEq)]
enum Position {
    #[serde(rename="Sr. Software Developer")]
    Sr_Software_Developer,
    #[serde(rename="Jr. Software Developer")]
    Jr_Software_Developer,
    #[serde(rename="Software Developer")]
    Software_Developer,
    #[serde(rename="Team Lead")]
    Team_Lead,
    #[serde(rename="Project Manager")]
    Project_Manager


}

fn main() {
    let data=fs::read_to_string("./src/Employee.json").expect("failed to read to string");
    let final_data:Vec<Employee>=serde_json::from_str(&data).expect("failed to Deserialize JSON");
    let mut mid_rust: Vec<Employee> = Vec::new();
    let mut jr_java: Vec<Employee> = Vec::new();
    let mut sr_c_hash: Vec<Employee> = Vec::new();


    for iter in final_data{
        if iter.position==Some(Position::Software_Developer) && 
        iter.skills.contains(&String::from("Rust")){
            mid_rust.push(iter);

        }
        else if iter.position==Some(Position::Jr_Software_Developer)&&
         iter.skills.contains(&String::from("Java")){
            jr_java.push(iter);

        }

        else if iter.position==Some(Position::Sr_Software_Developer) || 
       iter.skills.contains(&String::from("C#")){
            sr_c_hash.push(iter);

        }
    }

   let mid_rust= serde_json::to_string_pretty(&mid_rust).expect("failed during serializing to JSON");
   let jr_java= serde_json::to_string_pretty(&jr_java).expect("failed during serializing to JSON");
   let sr_c_hash= serde_json::to_string_pretty(&sr_c_hash).expect("failed during serializing to JSON");
    
    fs::write("./src/mid_rust.json", mid_rust);
    fs::write("./src/jr_java.json", jr_java);
    fs::write("./src/sr_c_hash.json", sr_c_hash);
    
    
}
