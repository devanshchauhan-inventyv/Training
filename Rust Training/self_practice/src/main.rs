use std::collections::HashMap;

// Given a list of integers, use a vector and return the median (when sorted, 
// the value in the middle position) and mode (the value that occurs most often;
//      a hash map will be helpful here) of the list.

fn main() {
    let mut list = vec![545,23,235,5,5,5,5,1,43,7,7,7,7,7,9,6,65,45,67,24,4];
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut  mode=0;

    list.sort();
    if !list.len()/2%2==0 {
       let median: f32= (list[list.len()/2]as f32+list[(list.len()/2)-1]as f32)/2 as f32;
       println!("The median is {}",median);

    }
    else {
        let median = list[(list.len()/2)];
        println!("The median is {}",median);
    }

    for i in 0..list.len(){
        let count=map.entry(list[i]).or_insert(0);
        *count+=1;
    }
    for i in map.values(){
        if mode<*i{
            mode=*i;
        }
    }
    for (key,value) in map{
        if value==mode{
            println!("{}",key);
            break;
        }
    }

    
}