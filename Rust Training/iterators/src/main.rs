mod main1;

fn main() {
//     let vec=vec![1,2,3];
//     let iter_vec=vec.iter();
//     println!("{:#?}",iter_vec);
//     type vec1=Vec<i32>;
//   let x=assert_eq!(vec,vec);
//   println!("{:#?}",x);

let vec: Vec<i32> = vec![1,2,3];

let result:Vec<i32> = vec.iter().map(|x|x+1).collect();

println!(" {:?}",vec);


}
