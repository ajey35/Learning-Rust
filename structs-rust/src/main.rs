#[derive(Debug)]
struct RightTriangle{
    base:f32,
    height:f32,
    hypo:f32
}

impl RightTriangle {
    fn area(&self)->f32{
        0.5 * self.base * self.height
    }
    fn can_hold(&self,other:RightTriangle)->bool{
        self.height>other.height
    }
}

impl RightTriangle{
    fn square(b:u32,l:u32)->u32{
        b*l
    }
}

fn main(){
    let t = RightTriangle{
        base:12.0,
        height:15.0,
        hypo:20.0
    };

    let t1 = RightTriangle{
        base:12.0,
        height:11.0,
        hypo:20.0
    };

    let t2 = RightTriangle{
        base:12.0,
        height:16.0,
        hypo:20.0
    };
    println!(" T = {:#?}",t);
    println!("Area of Triangle t : {}",t.area());

    println!("Can t hold t1 ? {}",t.can_hold(t1));
    println!("Can t hold t1 ? {}",t.can_hold(t2));

    // associated functionssss

    let rt = RightTriangle::square(12, 12);
    println!("Output from Associated FN : {rt}");
}