use rand::{thread_rng, Rng};

use crate::{EXECUTIVES, SKILLS, STATUS};

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
