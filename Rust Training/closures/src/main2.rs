use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // println!("{:?}",list); //here this println wont work as there is a mutable borrow
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // let list1 = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list1);

    // thread::spawn(move|| println!("From thread: {:?}", list1))
    //     .join()
    //     .unwrap();
}
