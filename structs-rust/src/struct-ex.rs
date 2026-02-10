

#[derive(Debug)]
struct User{
    email:String,
    name:String,
    active:bool,
    s_count:u32
}

struct Rect{
    l:u32,
    b:u32
}

fn build_user(email:String,name:String)->User{
    User { email, name, active:true, s_count: 1 }
}
fn mainnn() {
    let mut u1 = User{
        email:String::from("ajay@gmail.com"),
        name:String::from("Ajeya Kumara K"),
        active:true,
        s_count:1,
    };

    u1.name = String::from("Eleven");
    println!("User Name : {}",u1.name);

    let u2 = build_user(String::from("ajk@google.in"),String::from("Ajay"));

    println!("User 2 : {:#?}",u2);

    // We can also do something like this ,, mapping or copieing the another User attributes values to another User

    let u3 = User{
        email:String::from("Ajayyy@gmail.com"),
        ..u1
    };
    println!("U3 : {:#?}",u3);

    // Demo of Why do we need the structs for that , i have look for basic ex. Calculate the Area of Reactangle

    let l = 12;
    let b = 25;
    let area  = cal_area(l,b);
    println!("Area = {area}");

    let dim = (12,25);
    let area = cal_area_d(dim);
    println!("Area : {}",area);

    // now with the structsss.
    let r = Rect{
        l:12,
        b:25
    };
    println!("Area from struct : {}",cal_s_a(r));
}

fn cal_area(l:u32,b:u32)->u32{
   l*b
}

fn cal_area_d(dimentions:(u32,u32))->u32{
    dimentions.0 * dimentions.1
}

fn cal_s_a(r:Rect)->u32{
    r.b*r.l
}