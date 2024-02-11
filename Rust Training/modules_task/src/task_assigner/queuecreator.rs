use std::collections::VecDeque;

use crate::{ESCALATION_LEVEL, ESCALATION_QUEUE, LANGUAGE, REQUEST_TYPE, SKILLS};

pub fn creating_escalation_queue() {
    for request_type in 0..REQUEST_TYPE.len() {
        for skill in 0..SKILLS.len() {
            for language in 0..LANGUAGE.len() {
                for level in 0..ESCALATION_LEVEL.len() {
                    let key = format!(
                        "{}_{}_{}_{}",
                        REQUEST_TYPE[request_type],
                        SKILLS[skill],
                        LANGUAGE[language],
                        ESCALATION_LEVEL[level]
                    );
                    ESCALATION_QUEUE
                        .write()
                        .unwrap()
                        .insert(key, VecDeque::new());
                }
            }
        }
    }
}
