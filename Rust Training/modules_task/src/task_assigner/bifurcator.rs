use crate::{ESCALATION_QUEUE, PENDING_USER_QUEUE};
use chrono::Utc;

pub fn bifurcate() {
    if !PENDING_USER_QUEUE.read().unwrap().is_empty() {
        let mut level = String::new();
        let item = match PENDING_USER_QUEUE.write().unwrap().pop_front() {
            Some(val) => val,
            None => todo!("Some Problem in Pending User Queue"),
        };
        if Utc::now().timestamp() - item.timestamp < 30 {
            level = "L1".to_string();
        } else if Utc::now().timestamp() - item.timestamp < 60 {
            level = "L2".to_string();
        } else if Utc::now().timestamp() - item.timestamp < 90 {
            level = "L3".to_string();
        } else if Utc::now().timestamp() - item.timestamp < 120 {
            level = "L4".to_string();
        } else if Utc::now().timestamp() - item.timestamp < 150 {
            level = "L5".to_string();
        }
        let mut key = format!(
            "{}_{}_{}_{}",
            item.request_for, item.skills, item.language, level
        );

        println!("data bifurcated into {} queue", key);
        ESCALATION_QUEUE
            .write()
            .unwrap()
            .entry(key)
            .and_modify(|queue| {
                queue.push_back(item);
            });
    }
}
