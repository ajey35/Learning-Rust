// use std::{fmt::{Debug, Display}, iter::Sum};

// pub struct NewsArticle{
//     pub author:String,
//     pub headerline:String,
//     pub content:String
// }

// pub struct Tweet{
//     pub username:String,
//     pub content:String,
//     pub retweet:bool,
//     pub reply:bool
// }

// pub trait Summerize {
//     fn summary(&self)->String{
//         String::from("Reading More..")  // default implementation
//     }
    
// }

// impl Summerize for NewsArticle {
//     fn summary(&self)->String {
//         format!("{}, by {}",&self.headerline,&self.author)
//     }
// }

// impl Summerize for Tweet {}

// // fn notify(item:&impl Summerize){
// //     println!("{}",item.summary());
// // }
// // Same as Previous Notify Fn above only sytax change that's it! here you go you will understand how to use trait bounds for genericsssss

// // fn notify<T:Summerize>(item:&T){  // here we bounding the generic types to the trait Summerizeeee
// //     println!("{}",item.summary());
// // }

// fn notify(item1:&(impl Summerize + Display ),item2:&impl Summerize){
//     // ...
// }

// fn notify1<T:Summerize+Display>(item1:&T,item2:&T){
//     // ...
// }

// fn some_fn<T : Summerize + Display, U : Clone + Debug>(t:&T,u:&U)->i32{
//     1
// }
// // using where classsss
// fn some_fnn<T,U>(t:&T,u:&U)->i32
//   where T:Summerize + Display,
//   U: Clone + Debug
// {
//  12
// }


// fn main() {

//     let new1 = NewsArticle{
//         author:String::from("Ajey"),
//         headerline:String::from("Bitcoin Hits 1M Dollers"),
//         content:String::from("Hey Here you go bitcoin holders becomes millinears now they got now today morning 2027 May 1st Bitcoin Breaks all the record by securing 1M$ Bidd! ")
//     };

//     let t1 = Tweet{
//         username:String::from("Ajeya Kumara K"),
//         content:String::from("Bitcoin Hits 1M Dollers"),
//         retweet:true,
//         reply:false
//     };

//     // println!("{}",new1.summary());
//     // println!("{}",t1.summary());

//     // notify(&new1);
//     // notify(&t1);
// }

use std::fmt::Display;

struct Point<T>{
    x:T,
    y:T
}

impl<T> Point<T>{
    fn new(x:T,y:T)->Self{
        Point{
            x,y
        }
    }
}

impl<T:Display + PartialOrd > Point<T>{
    fn cmp_display(&self){

        if &self.x > &self.y {
            println!("X is Larger than Y");
        }
        else{
            println!("Y is Larger than X");
        }

    }
}

//Blanket Implementation!
//  Basically We can Implement the trait on a type that implements another trait !
impl<T:Display> ToString for T {
    // snip
}

fn main(){


}