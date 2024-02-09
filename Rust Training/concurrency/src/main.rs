use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T>Deref for MyBox<T>{
    type Target=T;
    fn deref(&self)->&Self::Target{
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}    

fn main() {
 let y=5;
 let x=  MyBox::new(y);

 assert_eq!(y,5);
 assert_eq!(*x,y);

 
}
