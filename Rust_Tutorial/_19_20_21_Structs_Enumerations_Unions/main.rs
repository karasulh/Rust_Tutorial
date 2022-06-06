#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use std::mem;

struct  Point
{
    x:f64,
    y:f64
}
struct Line
{
    start: Point,
    end: Point
}

fn structures(){
    let p = Point{ x: 3.0, y:4.0};
    println!("point p is at ({},{})", p.x,p.y);

    let p2 = Point{ x:5.0, y: 10.0};
    let myLine = Line {start:p , end:p2};
}


enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), //tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, //struct
}

fn enums(){
    //let c:Color = Color::Red;
    //let c:Color = Color::RgbColor(10,0,0);
    let c:Color = Color::CmykColor{cyan:0, magenta:128, yellow:0, black:255};

    match c
    {
        Color::Red => println!("RED"),
        Color::Blue => println!("BLUE"),
        Color::Green => println!("GREEN"),
        Color::RgbColor(0,0,0) | Color::CmykColor{cyan:_ ,magenta:_,yellow:_,black:255} => println!("Black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r,g,b),
        Color::CmykColor{cyan:cy,magenta:ma,yellow:ye,black:bl} => println!("cmyk {}, {}, {}, {}",cy,ma,ye,bl),
        _ => ()
    }
}

//32 bit
union IntOrFloat
{
    i:i32,
    f:f32
}

fn unions(){
    let mut iof= IntOrFloat{ i:123 };
    iof.i=234;

    let value = unsafe{ iof.i };
    println!("iof.i ={}",value);

}

fn unions_process_value(iof: IntOrFloat){
    unsafe{//should be in unsafe block, because we read variables in union
        match iof{
            IntOrFloat{i:42} => {println!("meaning of life value");} //it only takes when int=42
            IntOrFloat{f} => {println!("value = {}",f);} //it takes all float variables
        }
    }
}

fn main(){
    //structures();
    //enums();
    //unions();
    unions_process_value(IntOrFloat{i:42});
    unions_process_value(IntOrFloat{f:40.5});
    unions_process_value(IntOrFloat{i:5}); //It is treated as float number but the result is not seen correct.
}