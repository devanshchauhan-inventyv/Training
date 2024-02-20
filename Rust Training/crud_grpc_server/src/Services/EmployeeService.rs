use std::{fs, io, ptr::write_bytes};
use crate::EmpCrudMethods::{employee::EmployeeData, Employee_json};

pub fn read_employee_file() -> Result<Vec<Employee_json>, io::Error> {
    let val = fs::read_to_string("./src/data/Employee.json")?;
    let data: Vec<Employee_json> = serde_json::from_str(&val)?;
    Ok(data)
}

pub fn write_employee_file(data: &Vec<Employee_json>) -> Result<String, io::Error> {
    let val = serde_json::to_string_pretty(data)?;
    fs::write("./src/data/Employee.json", val)?;
    Ok("Data Added Successfully".to_string())
}

pub fn create_employee_data(emp: Employee_json) -> String {
    let mut main_data = Vec::new();
    match read_employee_file() {
        Ok(mut employee_vector) => {
            employee_vector.push(emp);
            main_data.extend_from_slice(&employee_vector);
        }
        Err(err) => {
            println!("{}", err);
            todo!("Failed to create employee");
        }
    }

    match write_employee_file(&main_data) {
        Ok(msg) => msg,
        Err(err) => format!("{}", err),
    }
}

pub fn read_employee_data(id:u32)->Result<Vec<Employee_json>,String>{
    let mut main_data:Vec<Employee_json>=vec![];
    match read_employee_file(){
        Ok(mut val)=>{
          val.retain(|item|item.id==id);
          Ok(val)
        },
        Err(err)=>Err(format!("{}",err).to_string())
    }
}

pub fn update_employee_data(id: u32, updated_employee: Employee_json) -> Result<String, String> {

    let mut data = match read_employee_file() {
        Ok(val) => val,
        Err(err) => return Err(format!("{}", err)),
    };

    if let Some(index) = data.iter().position(|emp| emp.id == id) {
        data[index] = updated_employee;
        match write_employee_file(&data) {
            Ok(_) => Ok("Employee data updated successfully".to_string()),
            Err(err) => Err(format!("{}", err)),
        }
    } else {
        Err("Employee not found".to_string())
    }
}

pub fn delete_employee_data(id: u32) -> Result<String, String> {
    let mut data = match read_employee_file() {
        Ok(val) => val,
        Err(err) => return Err(format!("{}", err)),
    };

    if let Some(index) = data.iter().position(|emp| emp.id == id) {
        data.remove(index);
        match write_employee_file(&data) {
            Ok(_) => Ok("Employee data deleted successfully".to_string()),
            Err(err) => Err(format!("{}", err)),
        }
    } else {
        Err("Employee not found".to_string())
    }
}