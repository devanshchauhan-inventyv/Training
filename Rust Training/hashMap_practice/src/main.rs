use std::collections::HashMap;

fn main() {
    let string="Hello world , wonderful world";
    let mut word_count_map= HashMap::new();
  

    for word in string.split_whitespace()  {
        let mut count: &mut u8= word_count_map.entry(word).or_insert(0);
        *count+=1;
    }


    println!("{:?}",word_count_map);
}
