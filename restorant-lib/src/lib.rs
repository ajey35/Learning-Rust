// mod front_house{
//     mod hosting{
//         fn add_to_waitlist(){}
//         fn see_at_table(){}
//     }
//     mod serving{
//         fn take_order(){}
//         fn serve_order(){}
//         fn take_payment(){}

//     }
// }
// -------------------------------------------------------
// mod front_house{
//     pub mod hosting{
//         pub fn make_order(){
//             println!("Order Set Please wait a Minute..");
//         }
//     }
// }

// fn at_restorant(){
//     // absolute path
//     crate::front_house::hosting::make_order();

//     // relative path
//     front_house::hosting::make_order();

// }
// ------------------------------------------------
// Lets Discuss about Super Keyword

// fn serve_order(){
//     println!("Serving Order...");
// }

// mod back_of_house{
//     fn fix_incorrect_order(){
//         cook_order();
//         super::serve_order(); // super refer to parent module which is "crate" in this case
//     }
//     fn cook_order(){
//         println!("Cooking Order....");
//     }
// }
// -------------------------------------------------------------
// privacy rules when comes to structs

// mod back_of_house{
//     #[derive(Debug)]
//     pub struct BreakFast{
//         toast:String,
//         season_fruit:String
//     }

//     impl BreakFast{
//         pub fn summer(toast:&str)->BreakFast{
//             BreakFast { toast: toast.to_string(), season_fruit:String::from("Mango") }
//         }
//     }
// }

// fn eat_at_resto(){
//     let mut meal = back_of_house::BreakFast::summer("toast");
//     println!("meal : {:#?}",meal);
// }

// -------------------------------------------------------------

// Lets Try for Enums

// mod back_of_house{
//     pub enum Gender{
//         Boy,
//         Girl,
//         Gay,
//         Lesbian
//     }
// }
// // once you make the enums public all of its variables becomes public as well
// // but it is not same in the case of structs , if you even though make the struct as public 
// // the variables inside the structs are not public , they are by default private!
// fn get_gender(){
//     let p1 = back_of_house::Gender::Boy;
//     let p2 = back_of_house::Gender::Girl;
//     let p3 = back_of_house::Gender::Lesbian;
// }

// ---------------------------------------------------------------
// Name conflicting!

// use std::io::Result as IResult;
// use std::fmt::Result;

// fn function1()->Result{
//     Ok(())
// }

// fn function2()->IResult<()>{
//     Ok(())
// }


// -----------------------------------------------


mod front_house;

// use crate::front_house::hosting; // absolute path
pub use crate::front_house::hosting; // relative path!

fn eat_at_resto(){
    // front_house::hosting::add_to_waitlist();  // hosting should be public to use here! & fn inside the hosting also pub
    hosting::add_to_waitlist(); // here no need to give full path bcz i imported!
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}