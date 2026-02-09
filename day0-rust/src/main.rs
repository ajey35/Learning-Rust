use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;
fn loopp(sn:i32){

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to Read Input");
    let gn:i32 = guess.trim().parse().expect("interger parse error");

    match gn.cmp(&sn){
        Ordering::Greater=>println!("{}","Too High".red()),
        Ordering::Less=>println!("{}","Too Less".red()),
        Ordering::Equal=>println!("{}","You Guessed Right and Exited".green())
    }
    if gn!=sn{
        loopp(sn);
    }
}

fn main(){

    let sn = rand::thread_rng().gen_range(0..10);
    println!("{}","Enter Your Guessed Number :".bright_blue());
    loopp(sn);

}