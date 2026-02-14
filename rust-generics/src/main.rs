//  Generics and Lifetimes always implemented to reduce the code-duplications
//  T is Generic it could be anything, anytype that we could not do math operations, inorder to fix this problem
// we need to restrict our generic type instead of saying our generic could be anytype we wanted to say our generic could be of any type that could be compared!!!
//  for this we need to use traitssss

// fn main() {
//     let vtr =  vec![12,45,37,89,32,108,123];

//     let  lx = get_largest(vtr);
    
//     println!("The Largest Number is : {lx}");

//     let vtr1 =  vec![-2,-1,0,1,2,3,4,11];

//     let lx1 = get_largest(vtr1);
    
//     println!("The Largest Number is : {lx1}");

//     let chvtr = vec!['a','j','a','y'];

//     let chlx = get_largest(chvtr);

//     println!("The Biggest Character is : {chlx}");

// }


// fn get_largest<T:PartialOrd + Copy >(vtr:Vec<T>)->T {   // implemented the trait bound!!!
//     let mut lx = vtr[0];

//     for num in vtr{
//         if num > lx{
//             lx = num;
//         }
//     }
//     lx
// }

// ---------------------------------------------------------------------

// // Generics with Structuressss
// Generics in Enumsss

// struct Point<T>{
//     x:T,
//     y:T
// }

// impl<U> Point<U> {
//     fn x(&self)->&U{
//         &self.x
//     }
// }
// impl Point<f32> {
//     fn y(&self)->&f32{
//         &self.y
//     }
// }

// fn main(){

//     let p1 = Point{
//         x:12,
//         y:13
//     };
//     let p2: Point<f32> = Point{
//         x:12.1,
//         y:13.5
//     };
//     p1.x();
//     p2.x();   
//     p2.y();

//     // let p3 = Point{
//     //     x:12,
//     //     y:13.5
//     // };
// }
// ---------------------------------------------------------
// #[derive(Debug)]
// struct Point<T,U>{
//     x:T,
//     y:U
// }

// impl<T,U> Point<T,U>  {
//     fn mix<V,W>(self,other:Point<V,W>)->Point<T,W>{
//         Point { x: self.x, y: other.y }
//     }
// }

// fn main(){
//     let p1 = Point{x:12,y:24};
//     let p2 = Point {x:"Hello",y:"Ajay"};
//     let p3 = p1.mix(p2);
//     println!("P3 : {:#?}",p3);
// }

// Generics in Enums especially option enums and Result Enuns

// Option Enum only has one generics
enum Option<T>{
    Some(T),
    None
}
// Result Enums has two Generic Variables
enum Result<T,E>{
    Ok(T),
    Err(E)
}

fn main(){
    let intt = Option::Some(11);
    let flott = Option::Some(11.1);
}