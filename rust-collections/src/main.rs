// fn main() {
//     let _a = [1,3,4,5,8,9,12];
//     let mut v:Vec<u32> = Vec::new();
//     v.push(1);
//     v.push(11);
//     v.push(111);
    
//     let mut  vtr = vec![1,2,3,5];
//     let val = &vtr[2];
//     println!("Third Element : {val}");
    
//     match vtr.get(21) {
//         Some(ans)=>println!("Third Index Value : {ans}"),
//         None=>println!("No Element is Present in Vector With That Index!")
//     }
//     // Iterating the Vector!
//     for i in &mut vtr{
//         *i *=10;
//     }
//     for i in &mut vtr{
//         println!("{i}");
//     }
// }

// fn main(){
//     enum SSC{
//         Integer(i32),
//         Floats(f32),
//         Text(String)
//     }

//     let sheet = vec![SSC::Integer(12),SSC::Floats(11.2),SSC::Text(String::from("hi"))];

//     match &sheet[0] {
//         SSC::Integer(i32)=>println!("It is Integer"),
//         _=>println!("It is Not a Integer")
//     }
// }


// -----------------------------------------------------
// Lets Talk about strings, strings are stored as a collection of UTF-8 encoded bytes stored in heap memory

// use unicode_segmentation::UnicodeSegmentation;  // imported for graphenes

// fn main(){

//     let mut s1 = String::new(); // empty string
//     let s2 = "string_slice";
//     let s3 = s2.to_string();
//     let s4 = String::from("HeyStringggg!");

//     // appending string fns!
//     s1.push_str("hi");  // this is for appending the string slices

//     s1.push('!');  // this is for appending the sigle character to the string!

//     let s5 =  s1 + &s4;  // here passing the ownership of s1 to s3 and appending the s4 
//     // println!("{s5}");
//     // println!("{s1}"); // cannot to use bcz s1 ownership transfered to s3 , so s1 no longer exists

//     let s1 = String::from("Hello, ");
//     let s2 = String::from(" World!");
//     let s3 = format!("{}{}",s1,s2);  // this won't take ownership
//     // println!("{s3}");
//     // println!("{s1}");  // this is valid bcz format macro wont take ownership
    
//     let s = String::from("मुझे तुमसे प्यार है");

//     for b in s.bytes(){
//         // println!("{b}");
//     }

//     for c in s.chars(){
//         // println!("{c}");
//     }
//     for g in s.graphemes(true){
//         println!("{g}");
//     }
// }


// // Now HashMapssss
// use std::collections::HashMap;
// fn main(){
//     let blue = String::from("blue");
//     let red = String::from("red");

//     let mut scores = HashMap::new();
//     // scores.insert(blue,9);
//     // scores.insert(red, 12);

//     // // println!("{}",blue); // getting error bcz we already passed ownership of the blue and red to hashmap!

//     // let team = String::from("red");
//     // let kp = scores.get(&team);  // returning an optional enum!

//     // for (key,value) in &scores{
//     //     println!("{} : {}",key,value);
//     // }

//     scores.insert(String::from("blue"), 12);
//     scores.insert(String::from("blue"), 22);


//     scores.entry(String::from("red")).or_insert(12);
//     scores.entry(String::from("red")).or_insert(122);

//     for (key,value) in &scores{
//         println!("{} : {}",key,value);
//     }
//     //output 
//     // { red : 12 , blue : 22 }
    
    
// }

// simple problem solving!
fn main(){
    let st = "hello world wonderfull world";
    let mut mp = HashMap::new();
    for word in st.split_whitespace(){
        let mut count = mp.entry(word).or_insert(0);  // here it returns to mutable refrence to the value of the  entry in hashmap so we can update it!
        *count +=1;
    }
    println!("{:#?}",mp);
}