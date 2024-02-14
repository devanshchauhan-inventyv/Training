use std::fs;

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    Customer_support, Employee, Message, Student, CUSTOMER_SERVICE, EMPLOYEE, STUDENT, TASK,
};

pub async fn load_data_to_queues() {
    match fs::read_to_string(format!("./src/axum_task/data/Employee.json")) {
        Ok(val) => {
            let data: Result<Vec<Employee>, serde_json::Error> = serde_json::from_str(&val);
            match data {
                Ok(final_data) => {
                    for emp in final_data.into_iter() {
                        EMPLOYEE.write().unwrap().push(emp);
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    todo!("Serialization Error")
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            todo!("Input Output Error")
        }
    }

    match fs::read_to_string(format!("./src/axum_task/data/StudentData.json")) {
        Ok(val) => {
            let data: Result<Vec<Student>, serde_json::Error> = serde_json::from_str(&val);
            match data {
                Ok(final_data) => {
                    for stud in final_data.into_iter() {
                        STUDENT.write().unwrap().push(stud);
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    todo!("Serialization Error")
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            todo!("Input Output Error")
        }
    }

    match fs::read_to_string(format!("./src/axum_task/data/Master_Data.json")) {
        Ok(val) => {
            let data: Result<Vec<Customer_support>, serde_json::Error> = serde_json::from_str(&val);
            match data {
                Ok(final_data) => {
                    for cust in final_data.into_iter() {
                        CUSTOMER_SERVICE.write().unwrap().push(cust);
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    todo!("Serialization Error")
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            todo!("Input Output Error")
        }
    }
}

pub async fn read_user(Path(task): Path<Vec<String>>) -> Response {
    if TASK.contains(&task[0]) {
        let string_id: String = task[1].clone();
        let task: String = task[0].clone();
        let id: usize = string_id.parse().unwrap();
        if task == "employee" {
            let data = EMPLOYEE.read().unwrap().clone();
            let emp_data:Vec<&Employee>=data.iter().filter(|&emp|emp.id==id).collect();
            if emp_data.is_empty() {
                Json(Message {
                    status: 4004,
                    message_key: String::from("error"),
                    data: "No employee found with the given ID".to_string(),
                }).into_response()
            } else {
                Json(Message {
                    status: 2000,
                    message_key: String::from("success"),
                    data: emp_data,
                }).into_response()
            }
        } else if task == "customer_service" {
            let data = CUSTOMER_SERVICE.read().unwrap().clone();
            let cust_data:Vec<&Customer_support>=data.iter().filter(|&cust|cust.id==id).collect();
            if cust_data.is_empty() {
                Json(Message {
                    status: 4004,
                    message_key: String::from("error"),
                    data: "No Customer_service employee found with the given ID".to_string(),
                }).into_response()
            } else {
                Json(Message {
                    status: 2000,
                    message_key: String::from("success"),
                    data: cust_data,
                }).into_response()
            }
        }
        else{
            let data= STUDENT.read().unwrap().clone();
            let stud_data:Vec<&Student>=data.iter().filter(|&stud|stud.id==id).collect();
            if stud_data.is_empty() {
                Json(Message {
                    status: 4004,
                    message_key: String::from("error"),
                    data: "No student found with the given ID".to_string(),
                }).into_response()
            } else {
                Json(Message {
                    status: 2000,
                    message_key: String::from("success"),
                    data: stud_data,
                }).into_response()
            }
        }
    
    } else{
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: "Data Path is incorrect".to_string(),
            })
            .into_response()
         }
}

pub async fn create_user_emp(Json(payload): Json<Employee>)->Response {
    
    let mut not_present=true;
   let data= EMPLOYEE.read().unwrap().clone();
   for emp in data{
       if payload.id==emp.id{
            not_present=false;
       }
   }
   if not_present{
    let emp=Employee{
        id: payload.id,
        name: payload.name,
        age: payload.age,
        skills: payload.skills,
        position: payload.position,
        experiance_y: payload.experiance_y,
    };
    EMPLOYEE.write().unwrap().push(emp.clone());
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: emp,
    }).into_response()

}else{
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: "employee with that ID already exist",
    }).into_response()

}

}
pub async fn create_user_stud(Json(payload): Json<Student>)->Response {
  
    let mut not_present=true;
   let data= STUDENT.read().unwrap().clone();
   for stud in data{
       if payload.id==stud.id{
            not_present=false;
       }
   }
   if not_present{
    let stud = Student{
        id: payload.id,
        name: payload.name,
        phone:payload.phone,
        email: payload.email,
        city: payload.city,
        address: payload.address,
        marks: payload.marks,
        percentage: payload.percentage,
        grade: payload.grade,
    };
    STUDENT.write().unwrap().push(stud.clone());
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: stud,
    }).into_response()

}else{
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: "student with that ID already exist",
    }).into_response()

}

}

pub async fn create_user_cust(Json(payload): Json<Customer_support>)->Response {
   
    let mut not_present=true;
   let data= CUSTOMER_SERVICE.read().unwrap().clone();
   for cust in data{
       if payload.id==cust.id{
            not_present=false;
       }
   }
   if not_present{
   let cust=Customer_support{
    id: payload.id,
    name: payload.name,
    skills: payload.skills,
    status: payload.status,
    language: payload.language,
};
    CUSTOMER_SERVICE.write().unwrap().push(cust.clone());
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: cust,
    }).into_response()

}else{
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: "customer service executive with that ID already exist",
    }).into_response()

}

}

pub async fn update_user_emp(Path(id):Path<usize>,Json(payload): Json<Employee>) ->Response{
    let mut updated=false;
   for emp in EMPLOYEE.write().unwrap().iter_mut(){
    if emp.id==id{
        emp.name=payload.name.clone();
        emp.age=payload.age.clone();
        emp.experiance_y=payload.experiance_y.clone();
        emp.position=payload.position.clone();
        emp.skills=payload.skills.clone();
        updated=true;
    }
   }
   if updated{
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: format!("data with ID {} has been updated",id),
    }).into_response()

   }else{
    let emp=Employee{
        id:id,
        name: payload.name.clone(),
        age:payload.age.clone() ,
        skills: payload.skills.clone(),
        position: payload.position.clone(),
        experiance_y: payload.experiance_y.clone(),
    };
    EMPLOYEE.write().unwrap().push(emp);
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: format!("New data with ID {} has been created as not found one",id),
    }).into_response()

   }
}
pub async fn update_user_stud(Path(id):Path<usize>,Json(payload): Json<Student>) ->Response{
    let mut updated=false;
   for stud in STUDENT.write().unwrap().iter_mut(){
    if stud.id==id{
        stud.address=payload.address.clone();
        stud.city=payload.city.clone();
        stud.email=payload.email.clone();
        stud.grade=payload.grade.clone();
        stud.marks=payload.marks.clone();
        stud.name=payload.name.clone();
        stud.percentage=payload.percentage.clone();
        stud.phone=payload.phone.clone();
        updated=true;
    }
   }
   if updated{
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: format!("data with ID {} has been updated",id),
    }).into_response()

   }else{
    let stud=Student{
        id:id,
        name: payload.name.clone(),
        phone: payload.phone.clone(),
        email: payload.email.clone(),
        city: payload.city.clone(),
        address: payload.address.clone(),
        marks: payload.marks.clone(),
        percentage: payload.percentage.clone(),
        grade: payload.grade.clone(),
    };
    STUDENT.write().unwrap().push(stud);
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: format!("New data with ID {} has been created as not found one",id),
    }).into_response()

   }
}

pub async fn update_user_cust(Path(id):Path<usize>,Json(payload): Json<Customer_support>) ->Response{
    let mut updated=false;
   for cust in CUSTOMER_SERVICE.write().unwrap().iter_mut(){
    if cust.id==id{
        cust.language=payload.language.clone();
        cust.name=payload.name.clone();
        cust.skills=payload.skills.clone();
        cust.status=payload.status.clone();
        updated=true;
    }
   }
   if updated{
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: format!("data with ID {} has been updated",id),
    }).into_response()

   }else{
    let cust =Customer_support{
        id:id,
        name: payload.name.clone(),
        skills: payload.skills.clone(),
        status: payload.status.clone(),
        language: payload.language.clone(),
    };
    CUSTOMER_SERVICE.write().unwrap().push(cust);
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: format!("New data with ID {} has been created as not found one",id),
    }).into_response()

   }
}
pub async fn delete_user(Path(task):Path<Vec<String>>)->Response {
    
    if TASK.contains(&task[0]) {
        load_data_to_queues().await;
        let string_id: String = task[1].clone();
        let task: String = task[0].clone();
        let id: usize = string_id.parse().unwrap();

    if task=="employee"{
     EMPLOYEE.write().unwrap().clone().retain(|item|item.id!=id);
     Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: format!("employee with ID {} has been deleted",id),
    }).into_response()

    }else if task =="student"{
        STUDENT.write().unwrap().retain(|item|item.id!=id);
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: format!("student with ID {} has been deleted",id),
        }).into_response()
    }else{
        CUSTOMER_SERVICE.write().unwrap().retain(|item|item.id!=id);
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: format!("cutomer service executive with ID {} has been deleted",id),
        }).into_response()
    }
}else{
    Json(Message {
        status: 4001,
        message_key: String::from("error"),
        data: "Data Path is incorrect".to_string(),
    })
    .into_response()
 }
}
pub async fn read_all_users(Path(task): Path<String>) -> Response {
    if TASK.contains(&task) {
        if task == "employee".to_string() {
            let data = EMPLOYEE.read().unwrap().clone();
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: data,
            })
            .into_response()
        } else if task == "student".to_string() {
            let data = STUDENT.read().unwrap().clone();
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: data,
            })
            .into_response()
        } else {
            let data = CUSTOMER_SERVICE.read().unwrap().clone();
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: data,
            })
            .into_response()
        }
    } else {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: "Data Path is incorrect".to_string(),
        })
        .into_response()
    }
}


// async fn write_data(data: &Vec<Employee>) {
//     let data = match serde_json::to_string_pretty(&data) {
//         Ok(data) => data,
//         Err(e) => {
//             println!("{e}");
//             todo!()
//         }
//     };
//     match fs::write("./src/data/Master_Data.json", data) {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e.to_string()),
//     };
// }

// async fn delete_user(Path(id): Path<i64>) -> String {
//     let delete_id = id;
//     let mut data = read_data().await;
//     data.retain(|emp| emp.id != id);
//     write_data(&data).await;
//     format!("the data with id {} is removed", delete_id)
// }

// async fn add_users(Json(data): Json<Employee>) -> impl IntoResponse {
//     let mut vector = read_data().await;
//     vector.push(data);
//     write_data(&vector).await;
//     Json(vector)
// }

// async fn update_user(Path(id): Path<i64>, Json(payload): Json<Employee>) -> Response {
//     let search_id = id;
//     let mut update = false;
//     let mut data = read_data().await;
//     for emp in data.iter_mut() {
//         if emp.id == search_id {
//             emp.name = payload.name.clone();
//             emp.skills = payload.skills.clone();
//             emp.status = payload.status.clone();
//             emp.language = payload.language.clone();
//             update = true;
//         }
//     }
//     write_data(&data).await;
//     if update {
//         Json(payload).into_response()
//     } else {
//         StatusCode::NOT_FOUND.into_response()
//     }
// }
