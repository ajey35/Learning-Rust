// fn main() {
//     // panic!("stop panicc");
//     //Use case of RUST_BACKTRACE=1 mode 
//     a();
// }

// fn a(){
//     b();
// }

// fn b(){
//     c(11);
// }
// fn c(num:u8){
//     if num==11{
//         panic!("Milli-Bobby-Brown!");
//     }
// }
// -------------------------------------------


use core::panic;
//Result Enums and Error handling!
use std::fs::File;
use std::io::ErrorKind;
fn main(){

    // enum Result<T,E>{
    //     Ok(T),
    //     Err(E)
    // }

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file)=>file,
        Err(err)=> match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file)=>file,
                Err(err)=>panic!("Problem in creating file : {:?}",e)
            },
            other_error => {
                panic!("problem opening file : {:?}",other_error)
            }
        }
    };

}
