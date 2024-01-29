#[derive(Debug)]
struct Point<T,U>{
    x : T,
    y : U
}

impl <T,U> Point<T,U> {
    fn x(self)->T{
        self.x
    }
    fn mixup<X2,Y2>(self,other:Point<X2,Y2>)->Point<T,Y2>{
        Point{
            x:self.x ,
            y:other.y
        }   

    }
    
}
// impl Point<f32,f32>{
//     fn distance_from_origin(&self)->f32{
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }



fn main() {
    //using generics and a getter of x 
    // let p = Point{x:3.5666868584989786859657,y:5};
    // println!("p.x = {}",p.x());
    
    //Using a concept that I can decalre a impl for a specific type too, for a generic struct 
    //let p1: Point<f32, f32> = Point{x:4.5675,y:7.8963};
    //println!("{}",p1.distance_from_origin());

    //this is a mixup of generics type of two different generic point instance while having a single a point struct
    // let p2=Point{x:5.0,y:3};
    // let other=Point{x:"hello",y:'c'};
    // println!("{:?}", p2.mixup(other));
    let v=vec![1,2,3,4,5,6];
    println!("{:?}",v);
}
