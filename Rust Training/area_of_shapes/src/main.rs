#[derive(Debug)]
struct Measurement{
    length:f32,
    breath:f32,
   
}
impl Measurement{
    fn can_hold(&self,obj:&Measurement)->bool{
        if self.length>obj.length && self.breath>obj.breath {
             true
        }
        else {
            false
        }

    }
    fn area_of_rectangle(&self)->f32{
        self.length*self.breath

    }

    fn area_of_square(&self)->f32{
        self.length*self.breath

    }

    fn area_of_circle(&self)->f32{
        (self.length*self.length)/(4.0*3.14159)
    }
}
fn main() {
    
    let measurement1=Measurement{
        length:10.0,
        breath:6.5,
        
    };
    
    println!("{}",measurement1.area_of_rectangle());
    println!("{}",measurement1.area_of_square());
    println!("{}",measurement1.area_of_circle());
    println!("{:?}",measurement1);

}
