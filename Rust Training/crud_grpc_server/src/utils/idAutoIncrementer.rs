use std::sync::Mutex;
use lazy_static::lazy_static;


pub fn idAutoIncrementer()->u32{
    let mut id=ID.lock().unwrap();
    let value=id.clone();
    *id+=1;
    value
}
lazy_static!{
    pub static ref ID:Mutex<u32>=Mutex::new(50);
}

