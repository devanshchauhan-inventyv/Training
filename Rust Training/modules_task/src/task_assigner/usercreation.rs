use std::fs;

use chrono::Utc;
use rand::{thread_rng, Rng};

use crate::{Customer_support, User_Request, LANGUAGE, REQUEST_TYPE, SKILLS};

pub fn read_executives() -> Result<Vec<Customer_support>, String> {
    let support_staff = fs::read_to_string("./src/task_assigner/data/Master_Data.json")
        .map_err(|err| format!("Error reading file: {}", err))?;

    serde_json::from_str(&support_staff).map_err(|err| format!("Error parsing JSON: {}", err))
}

pub fn generate_user() -> User_Request {
    let skills = SKILLS[thread_rng().gen_range(0..SKILLS.len())].to_string();
    let language = LANGUAGE[thread_rng().gen_range(0..=1)].to_string();
    let request_for = REQUEST_TYPE[thread_rng().gen_range(0..=1)].to_string();

    let timestamp = Utc::now().timestamp();
    User_Request {
        skills,
        language,
        request_for,
        timestamp,
    }
}
