fn main() {
    const NUM:u8 = 122;
    // const NAME:u32 = 12;  cannot able to do shadowing & initlize as mut var to constants 

    let x = 12;
    println!("X : {}",x);
    let x = 122;   // shadowing the var x
    println!("X : {}",x);
    let x:u32 = 1222;
    println!("X : {}",x); // changing the type of var x by using shadowing

    /*
       Scaler Datatypes 
       1.Intergers
       2.Floating Numbers
       3.Booleans
       4.Characters
    */
    // Integer (Diff way rep integers)
    let a = 98_222;  //Dec
    let a = 0xff; //hex
    let a = 0o77; // oct
    let a  = 0b1111_000; // bin
    let a = b'A';  // Byte (u8 only)

    let f:u8 = 255;

    // TUPLES
    let tup = (1,"Ajay");
    let (sl,name) = tup;
    println!("Name : {}",tup.1);

    let arr = [1,2,4,5,6,7];
    let byte  = [0;5];
    println!("{:?}",byte);

    // controlled assigment
    let condition = true;
    let num = if condition { 6} else {4};

    // control flowww
    let mut cnt= 0;
    let vl = loop{
        cnt+=1;
        if cnt==10{
            break cnt; 
        }
    };
    println!("Result : {}",vl);
    // while loop
    let mut cnn = 3;
    while cnn!=0{
        println!("{}",cnn);
        cnn-=1;

    }
    println!("LIFFF--");
    // for loop
    let arr = [10,20,30,50,60,70];
    for i in arr{
        println!("num {}",i);
    }
    for i in arr.iter(){
        println!("{}",i);
    }
    for number in 1..10{
        println!("{}",number);
    }

    // Line comment
    /* 
      Block Line Commentttttttttt
    */
}
