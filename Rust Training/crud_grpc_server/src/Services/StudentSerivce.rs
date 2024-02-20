use std::{fs, io};
use crate::StudCrudMethod::Student_json;


pub fn read_student_file() -> Result<Vec<Student_json>, io::Error> {
    let val = fs::read_to_string("./src/data/StudentData.json")?;
    let data: Vec<Student_json> = serde_json::from_str(&val)?;
    Ok(data)
}

pub fn write_student_file(data: &Vec<Student_json>) -> Result<String, io::Error> {
    let val = serde_json::to_string_pretty(data)?;
    fs::write("./src/data/StudentData.json", val)?;
    Ok("Data Added Successfully".to_string())
}

pub fn create_student_data(stud: Student_json) -> String {
    let mut main_data = Vec::new();
    match read_student_file() {
        Ok(mut student_vector) => {
            let mut already_present=false;
            for item in student_vector.iter(){
                if item.id==stud.id{
                    already_present=true;
                }
            }
            if !already_present{
            student_vector.push(stud);
            main_data.extend_from_slice(&student_vector);
            }
        }
        Err(err) => {
            println!("{}", err);
            todo!("Failed to create student");
        }
    }

    match write_student_file(&main_data) {
        Ok(msg) => msg,
        Err(err) => format!("{}", err),
    }
}

pub fn read_student_data(id:u32)->Result<Vec<Student_json>,String>{
    let mut main_data:Vec<Student_json>=vec![];
    match read_student_file(){
        Ok(mut val)=>{
          val.retain(|item|item.id==id);
          Ok(val)
        },
        Err(err)=>Err(format!("{}",err).to_string())
    }
}

pub fn update_student_data(id: u32, updated_student: Student_json) -> Result<String, String> {

    let mut data = match read_student_file() {
        Ok(val) => val,
        Err(err) => return Err(format!("{}", err)),
    };

    if let Some(index) = data.iter().position(|emp| emp.id == id) {
        data[index] = updated_student;
        match write_student_file(&data) {
            Ok(_) => Ok("Employee data updated successfully".to_string()),
            Err(err) => Err(format!("{}", err)),
        }
    } else {
        Err("student not found".to_string())
    }
}

pub fn delete_student_data(id: u64) -> Result<String, String> {
    let mut data = match read_student_file() {
        Ok(val) => val,
        Err(err) => return Err(format!("{}", err)),
    };

    if let Some(index) = data.iter().position(|stud| stud.id == id as u32) {
        data.remove(index);
        match write_student_file(&data) {
            Ok(_) => Ok("Employee data deleted successfully".to_string()),
            Err(err) => Err(format!("{}", err)),
        }
    } else {
        Err("Employee not found".to_string())
    }
}