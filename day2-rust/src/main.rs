// Ownerships in Rust Launguage!

/*
  1. Each Value in the Rust has Variable that's called it Owner.
  2. There can only be one owner at a time
  3. When the owner goes out  of scope, the value will be dropped..

  Note :
  1. You can only have one mutable refrance for a perticular piece of data in the perticular scope 
  2. You cannot have a mutable  refrance if an immutable refrance already exists (immutable refs don't expect underlying value to change)
*/
fn main() {
    let str = String::from("hello world");
    let hello  = &str[..5];
    let world = &str[5..];
}
