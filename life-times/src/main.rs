//  Below code get Lifetime Error Leads to Dangling Pointer Error!

// fn main() {
    
//     let r;               // ------------------+-------------- 'a
//     {                          //                   | 
//         let x = 5;        //------+---- 'b     |              
//         r = &x;                //      |            |  
//     }                          //------+            |
//                                //                   |
//     println!("r : {}",r);      //                   |
// }                              //-------------------+                

// Below Code Does not get any Lifetime error bcz both r & x lifetimes ends at same point!

// fn main() {
    
//     let r;               // ------------------+-------------- 'a
//                                //                   | 
//     let x = 5;            //------+---- 'b     |              
//     r = &x;                    //      |            |  
//                                //      |            |
//                                //      |            |
//     println!("r : {}",r);      //      |            |
// }                              //------+------------+              


// fn main(){
//     let string1 = String::from("abcd");
//     let string2 = String::from("xyz");

//     let result = longest(string1.as_str(),string2.as_str());
//     println!("The Longest String is {} ",result);
// }

// // &i32          // a refrance
// // &'a i32       // a refrance  with an explicit lifetime
// // &'a mut i32   // a mutable refrance with an explicit lifetime!

// fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
//     if x.len() > y.len(){
//         x
//     }else{
//         y
//     }
// }

// fn main(){
//     let string1 = String::from("abcd");
//     let result;
//     {
//         let string2 = String::from("xyz");
        
//         result = longest(string1.as_str(),string2.as_str());  // getting error bcz string2 does not live long enough!(means having smaller lifetime!)
 
//     }
//     println!("The Longest String is {} ",result);
// }

// fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
//     if x.len() > y.len(){
//         x
//     }else{
//         y
//     }
// }

// fn main(){
//     let string1 = String::from("abcd");
//     let result;
//     {
//         let string2 = String::from("xyz");
        
//         result = longest(string1.as_str(),string2.as_str());  // error gone bcz im only returning string1 , by mentioning that return type lifetime is always string1's
 
//     }
//     println!("The Longest String is {} ",result);
// }

// fn longest<'a>(x:&'a str,y:&str)->&'a str{   // no matter what ? im returning the string1's lifetimee
//    x
// }


// Lifetimes in Structsss

// struct ImpExcerpt<'a>{
//     part:&'a str
// }

// fn main(){
//     let first_sentence = String::from("You Take the Man out of the city");
//     let i = ImpExcerpt{
//         part:&first_sentence
//     };
// }

// RULES OF LIFE_TIMESS!
// --------------------+
// 1.  Each Parameter  that is a reference gets its own lifetime parameter
// 2. If there is exactly one input lifetime parameter, that lifetime is 
//    assigned  to all output lifetime parameters!
// 3. If there are multiple input lifetime parameters, but one of them is &self or
//    &mut self the lifetime of self is assigned to all output lifetime parameters.

//  Example for 3rd Rule.....
// struct ImpExcerpt<'a>{
//     part:&'a str
// }

// impl<'a> ImpExcerpt<'a> {
//     fn return_part(&'a self,announcement:&str)->&'a str{
//         println!("Attention Please : {}",announcement);
//         self.part
//     }
// }

// fn main(){
//     let first_sentence = String::from("You Take the Man out of the city");
//     let i = ImpExcerpt{
//         part:&first_sentence
//     };
// }

// STATIC - LIFETIMES! :
// -- static lifetimes means that the refrences could live as long as the duration of the program!
// fn main(){
//     // all string literals have a static lifetime!
//     // bcz they are stored in program binary so they could live the duration of the program!
//     let s:&'static str = "I have a static lifetime!";
// }

// One Final Example Where you Will see all "GENERICS + TRAITS + LIFETIMES"

use std::fmt::Display;

fn longest_with_ann<'a,T>(x:&'a str,y:&'a str,announcement:T)->&'a str
   where T:Display
{
    println!("Announcement : {}",announcement);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

fn main(){
    let ann = String::from("Hey Completely Understood Rust Lifetimes and Generics + Traits thanks Let's-Get's-Rusty!");
    let longg = longest_with_ann("hiiajay", "helloajaykumar", ann);
    println!("{longg}");
}