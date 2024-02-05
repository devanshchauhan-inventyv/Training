use crate::{
    Customer_support, User_Request, CALL_USER_QUEUE, CHAT_USER_QUEUE, EXECUTIVES, LANGUAGE,
    PENDING_USER_QUEUE, REQUEST_TYPE, SKILLS, STATUS,
};
use rand::{thread_rng, Rng};
use std::{fs, thread, time::Duration};

pub fn read_executives() -> Result<Vec<Customer_support>, String> {
    let support_staff = fs::read_to_string("./src/task_assigner/data/Master_Data.json")
        .map_err(|err| format!("Error reading file: {}", err))?;

    serde_json::from_str(&support_staff).map_err(|err| format!("Error parsing JSON: {}", err))
}

pub fn generate_user() -> User_Request {
    let skills = SKILLS[thread_rng().gen_range(0..SKILLS.len())].to_string();
    let language = LANGUAGE[thread_rng().gen_range(0..=1)].to_string();
    let request_for = REQUEST_TYPE[thread_rng().gen_range(0..=1)].to_string();
    User_Request {
        skills,
        language,
        request_for,
    }
}

pub fn executives_skill_changer() {
    let random_index = thread_rng().gen_range(0..EXECUTIVES.read().unwrap().len());
    let mut skills: Vec<String> = Vec::new();
    let random_number = thread_rng().gen_range(1..=3);
    for i in 0..random_number {
        skills.push(SKILLS[thread_rng().gen_range(0..SKILLS.len())].to_string());
    }
    EXECUTIVES.write().unwrap()[random_index].skills = skills;
    println!("skills changes of a user");
}

pub fn executives_status_changer() {
    let random_index = thread_rng().gen_range(0..EXECUTIVES.read().unwrap().len());
    let random_status = STATUS[thread_rng().gen_range(0..=1)].to_string();
    EXECUTIVES.write().unwrap()[random_index].status = random_status;
    println!("status changed of a user");
}

pub fn executives_language_changer() {
    let random_index = thread_rng().gen_range(0..EXECUTIVES.read().unwrap().len());
    let random_language = STATUS[thread_rng().gen_range(0..=1)].to_string();
    EXECUTIVES.write().unwrap()[random_index].language = random_language;
    println!("language changed of a user");
}

pub fn bifurcate() {
    let item = match PENDING_USER_QUEUE.write().unwrap().pop_back() {
        Some(val) => val,
        None => todo!(),
    };
    if item.request_for == "Incoming Call".to_string() {
        CALL_USER_QUEUE.write().unwrap().push_front(item);
        println!("data bifurcated into call");
    } else {
        CHAT_USER_QUEUE.write().unwrap().push_front(item);
        println!("data bifurcated into chat");
    }
}

pub fn decision_maker_for_chat() {
    if !CHAT_USER_QUEUE.read().unwrap().is_empty(){
    let mut flag = true;
    let user_to_be_checked = match CHAT_USER_QUEUE.write().unwrap().pop_back() {
        Some(val) => val,
        None => todo!("chat IS EMPTY")
    };

    for i in 0..EXECUTIVES.read().unwrap().len() {
        if EXECUTIVES.read().unwrap()[i].status == "Online".to_string() {
            if EXECUTIVES.read().unwrap()[i].language == user_to_be_checked.language {
                if EXECUTIVES.read().unwrap()[i]
                    .skills
                    .contains(&user_to_be_checked.skills)
                {
                    flag = false;
                    println!("THE USER_REQUEST WAS MATCHED AND DATA POPED");
                   
                }
            }
        }
    }
    if flag == true {
        CHAT_USER_QUEUE
            .write()
            .unwrap()
            .push_front(user_to_be_checked);
        println!("THE USER_REQUEST WAS NOT MATCHED AND DATA WAS ADDED BACK IN THE CHAT QUEUE");
    }
}else{
    thread::sleep(Duration::from_secs(7));
}
}

pub fn decision_maker_for_call() {
    if !CALL_USER_QUEUE.read().unwrap().is_empty(){
    let mut flag = true;
    let user_to_be_checked = match CALL_USER_QUEUE.write().unwrap().pop_back() {
        Some(val) => val,
        None => todo!("call is empty"),
    };

    for i in 0..EXECUTIVES.read().unwrap().len() {
        if EXECUTIVES.read().unwrap()[i].status == "Online".to_string() {
            if EXECUTIVES.read().unwrap()[i].language == user_to_be_checked.language {
                if EXECUTIVES.read().unwrap()[i]
                    .skills
                    .contains(&user_to_be_checked.skills)
                {
                    flag = false;
                    println!("THE USER_REQUEST WAS MATCHED AND DATA POPED")
                }
            }
        }
    }
    if flag == true {
        CALL_USER_QUEUE
            .write()
            .unwrap()
            .push_front(user_to_be_checked);
        println!("THE USER_REQUEST WAS NOT MATCHED AND DATA WAS ADDED BACK IN THE CALL QUEUE");
    }
}else{
    thread::sleep(Duration::from_secs(7));
}
}
