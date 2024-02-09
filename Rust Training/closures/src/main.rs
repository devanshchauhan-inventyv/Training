mod main1;
use main1::*;
mod main2;
use main2::*;
/*A factory is giving away tshirts to their user and if the user have a prefrence and
if that tshirt color is available then they give that or else the most stocked
tshirt */

#[derive(Debug, PartialEq, Clone, Copy)]
enum tShirtColor {
    Red,
    Blue,
}
#[derive(Debug)]
struct Inventory {
    tshirts: Vec<tShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_prefrences: Option<tShirtColor>) -> tShirtColor {
        user_prefrences.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> tShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for tShirt in &self.tshirts {
            match tShirt {
                tShirtColor::Red => num_red += 1,
                tShirtColor::Blue => num_blue += 1,
            };
        }
        if num_red > num_blue {
            tShirtColor::Red
        } else {
            tShirtColor::Blue
        }
    }
}
fn main() {
    //     let store=Inventory{
    //         tshirts:vec![tShirtColor::Blue,tShirtColor::Red,tShirtColor::Blue]
    //     };
    //     let user1_prefrence=Some(tShirtColor::Red);
    //     let result1=store.giveaway(user1_prefrence);
    //     println!("the user had preference {:#?} and got {:#?}",user1_prefrence,result1);
    //     let user2_prefrence=None;
    //    let result2= store.giveaway(user2_prefrence);
    //    println!("the user had preference {:?} and got {:#?}",user2_prefrence,result2);

    // main1::main_1();

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    let answer = add_one_v2(add_one_v3(add_one_v4(add_one_v1(0))));
    println!("{}", answer);
}
