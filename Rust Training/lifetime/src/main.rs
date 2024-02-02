fn main() {
    let str1="abcde";
    let r=6;
    let q="hello";
    let str2="xyz";
  let result=longest(str1.as_ptr(), str2.as_ptr());
    let x= str1.as_ptr();
    let y=str2.as_ptr();
    if result==x{
        println!("{}",str1);
    }else{
        println!("{}",str2);
    }
}

fn longest(str1:*const u8,str2:*const u8)->*const u8{
    if str1>str2{
        str2
    }
    else{
        str1
    }
}