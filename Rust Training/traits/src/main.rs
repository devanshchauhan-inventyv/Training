use std::fmt::Display;

use traits::*;



fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    // using the trait summary and calling the summarize method
    // println!("1 new tweet: {}", tweet.summarize());

    //we can also pass trait as paramters for differet function for example 
    // fn notify(item:&impl Summary){
    //     println!("notify method = {}",item.summarize())
    // }
    let item = Tweet{
        username:String::from("devansh"),
        content:String::from("What I learn , I learn that throughly and in deep"),
        reply:true,
        retweet:false,
    };
    //calling notify method on item instance of tweet
    // notify(&item);

    //this type of declaration is more verbose than using syntax sugar 
    //the trait bound syntax for notify() is
    // fn notify < T:Summary > (item:&T){
    //     println!("trait bound syntax for notify method = {}",item.summarize())
    // }

    //calling notify method on item instance of tweet
    // notify(&item);

    //trait bound having more paramters
    /*here the method takes 2 parameters but the datatype of both can be different but both
      only constraint is both datatype should implement summary trait */ 
    // fn notify ( item1:&impl Summary, item2:&impl Summary ) {

    //     println!("notify method = item1: {} and item2: {}",item1.summarize(),item2.summarize())
    // }

    //but to impose more strict contraints we can use the trait bound syntax
    // fn notify<T:Summary>(item1:&T,item2:&T){
    //     //here the dtatatype of both item1 and item2 shoulb be same
    //     println!("notify method = item1: {} and item2: {}",item1.summarize(),item2.summarize())
    // }

    /*this will not compile when you use the notify method in line 58
      for this to compile use the notify mathod in line 52*/
    // notify(&tweet, &article);

    
        //implementing trait bound to apply specific method to specific datatype
        // let pair1= Pair::new(5.99, 5.98);
        // Pair::cmp_both(&pair1);
        

    
}

struct Pair<T>{
    x:T,
    y:T,
}
impl<T> Pair<T>{
    fn new(x:T , y:T)-> Self{
        Self{x,y}
    }
}

impl <T:Display + PartialOrd> Pair<T>{
    fn cmp_both(&self){
        if self.x>self.y{
          println!("the bigger from both is x : {}",self.x);
        }
        else if self.y>self.x{
            println!("the bigger from both is x : {}",self.y);
        }
        else {
            println!("Both are equal x and y and their value is : {}",self.x);

        }
    }

}