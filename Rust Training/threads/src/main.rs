use std::{sync::mpsc::{self, Receiver}, thread::{self, Thread}, time::Duration};

fn main() {
//    let thread1= thread::spawn(||{
//                 for i in 1..10{
//             println!("hi {} from spwaned thread",i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     thread1.join().unwrap();
//     for i in 1..8{
//         println!("hi {} from main thread",i);
        // }

    // let mut v: Vec<&'static i32>=vec![&1,&2,&3,&4];
    // let x=thread::spawn(move ||{
    //         println!("the vector in thread used has a value {:?}",v);
    // });
    // x.join();
    // println!("{:#?}",v);
    let (tx,rx)=mpsc::channel();
    thread::spawn(move||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("devansh")];

            for val in vals{
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(2))
            }
    });

    for received in rx{
        println!("{}",received);
    }





}
