use std::{thread, time::Duration};

use crate::{CALL_USER_QUEUE, CHAT_USER_QUEUE, ESCALATION_LEVEL, ESCALATION_QUEUE, EXECUTIVES};

pub fn decision_maker() {
    for level in ESCALATION_LEVEL.iter() {
        for (value, item) in ESCALATION_QUEUE.write().unwrap().iter_mut() {
            if value.contains(level) {
                if !item.is_empty() {
                    let mut flag = true;
                    for user_request in item.pop_front().into_iter() {
                        for i in 0..EXECUTIVES.read().unwrap().len() {
                            if EXECUTIVES.read().unwrap()[i].status == "Online".to_string()
                                && EXECUTIVES.read().unwrap()[i].language == user_request.language
                                && EXECUTIVES.read().unwrap()[i]
                                    .skills
                                    .contains(&user_request.skills)
                            {
                                flag = false;
                                println!("THE USER_REQUEST WAS MATCHED AND DATA WAS POPED");
                            }
                        }
                        if flag == true {
                            item.push_back(user_request);
                            println!("THE USER_REQUEST WAS NOT MATCHED AND DATA WAS REMAINED IN THE QUEUE");
                        }
                    }
                }
            }
        }
    }
}

pub fn decision_maker_for_chat() {
    if !CHAT_USER_QUEUE.read().unwrap().is_empty() {
        let mut flag = true;
        let user_to_be_checked = match CHAT_USER_QUEUE.write().unwrap().pop_back() {
            Some(val) => val,
            None => todo!("chat IS EMPTY"),
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
    } else {
        thread::sleep(Duration::from_secs(7));
    }
}

pub fn decision_maker_for_call() {
    if !CALL_USER_QUEUE.read().unwrap().is_empty() {
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
    } else {
        thread::sleep(Duration::from_secs(7));
    }
}
