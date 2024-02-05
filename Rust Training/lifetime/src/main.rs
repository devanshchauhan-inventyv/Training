use std::fmt::Display;

fn main() {
    let string1 = String::from("hello this is string1");
    // let x=5;//this will works as its a stack data
    let slice1 = String::from("hello");
    // let result;
    // let novel = String::from("hello this is a novel. well this line wont come ");
    // let firstline=novel.split(".").next().expect("cannot find '.'");
    // let i = ImportantExcerpt{
    //     part:firstline
    // };
    let result_of_longest_with_announcment =
        longest_with_announcment(string1.as_str(), slice1.as_str(), "this is announcment");
    println!("{}", result_of_longest_with_announcment);

    // println!("{:#?}",i);
    {
        // let string2 = String::from("hi this is string2");
        // let y=6;//this will works as its a stack data
        //    let slice2=String::from("hii");
        // result = biggest(x, y);
        // result = longest(&slice1, &slice2);
        // println!("{}",result);//this will work
    }
    // println!("{}",result);//this wont work
}
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}
fn biggest(string1: i32, string2: i32) -> i32 {
    if string1 > string2 {
        string1
    } else {
        string2
    }
}

fn longest_with_announcment<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
