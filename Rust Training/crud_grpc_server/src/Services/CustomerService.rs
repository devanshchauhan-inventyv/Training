use std::{fs, io};

use crate::CustServiceCrudMethod::Customer_support;


pub fn read_customerService_file() -> Result<Vec<Customer_support>, io::Error> {
    let val = fs::read_to_string("./src/data/Master_Data.json")?;
    let data: Vec<Customer_support> = serde_json::from_str(&val)?;
    Ok(data)
}

pub fn write_customerService_file(data: &Vec<Customer_support>) -> Result<String, io::Error> {
    let val = serde_json::to_string_pretty(data)?;
    fs::write("./src/data/Master_Data.json", val)?;
    Ok("Data Added Successfully".to_string())
}

pub fn create_customerService_data(emp: Customer_support) -> String {
    let mut main_data = Vec::new();
    match read_customerService_file() {
        Ok(mut customerService_vector) => {
            customerService_vector.push(emp);
            main_data.extend_from_slice(&customerService_vector);
        }
        Err(err) => {
            println!("{}", err);
            todo!("Failed to create customerService");
        }
    }

    match write_customerService_file(&main_data) {
        Ok(msg) => msg,
        Err(err) => format!("{}", err),
    }
}

pub fn read_customerService_data(id:u32)->Result<Vec<Customer_support>,String>{
    let mut main_data:Vec<Customer_support>=vec![];
    match read_customerService_file(){
        Ok(mut val)=>{
          val.retain(|item|item.id==id);
          Ok(val)
        },
        Err(err)=>Err(format!("{}",err).to_string())
    }
}

pub fn update_customerService_data(id: u32, updated_customerService: Customer_support) -> Result<String, String> {

    let mut data = match read_customerService_file() {
        Ok(val) => val,
        Err(err) => return Err(format!("{}", err)),
    };

    if let Some(index) = data.iter().position(|emp| emp.id == id) {
        data[index] = updated_customerService;
        match write_customerService_file(&data) {
            Ok(_) => Ok("customerService data updated successfully".to_string()),
            Err(err) => Err(format!("{}", err)),
        }
    } else {
        Err("customerService not found".to_string())
    }
}

pub fn delete_customerService_data(id: u32) -> Result<String, String> {
    let mut data = match read_customerService_file() {
        Ok(val) => val,
        Err(err) => return Err(format!("{}", err)),
    };

    if let Some(index) = data.iter().position(|emp| emp.id == id) {
        data.remove(index);
        match write_customerService_file(&data) {
            Ok(_) => Ok("customerService data deleted successfully".to_string()),
            Err(err) => Err(format!("{}", err)),
        }
    } else {
        Err("customerService not found".to_string())
    }
}