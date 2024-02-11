use std::collections::{HashMap, VecDeque};

use chrono::Utc;

use crate::ESCALATION_QUEUE;

pub fn escalation_time_monitor() {
    let mut hmap = HashMap::new();
    for (value, item) in ESCALATION_QUEUE.write().unwrap().iter_mut() {
        if !item.is_empty() {
            let mut new_key_name: Option<String> = None;
            let mut list_of_req = VecDeque::new();
            for user_request in item.pop_front().into_iter() {
                let mut level = String::new();
                if Utc::now().timestamp() - user_request.timestamp < 30 {
                    level.push_str("L1");
                } else if Utc::now().timestamp() - user_request.timestamp < 60 {
                    level.push_str("L2");
                } else if Utc::now().timestamp() - user_request.timestamp < 90 {
                    level.push_str("L3");
                } else if Utc::now().timestamp() - user_request.timestamp < 120 {
                    level.push_str("L4");
                } else if Utc::now().timestamp() - user_request.timestamp < 150 {
                    level.push_str("L5");
                }
                let key = format!(
                    "{}_{}_{}_{}",
                    user_request.request_for, user_request.skills, user_request.language, level
                );
                println!("request escalated into {}", key);
                new_key_name = Some(key);
                list_of_req.push_back(user_request);
            }
            hmap.insert(new_key_name, list_of_req);
        }
    }
    for (each_key, mut req) in hmap.into_iter() {
        ESCALATION_QUEUE
            .write()
            .unwrap()
            .entry(each_key.unwrap())
            .and_modify(|queue| {
                queue.append(&mut req);
            });
    }
}
