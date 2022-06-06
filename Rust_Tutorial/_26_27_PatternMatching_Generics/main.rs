fn pm_how_many(x:i32) -> &'static str //we should define return as &'static str
{
    
    match x
    {
        0=> "no",
        1|2 => "one or two",
        12 => "dozen",
        z @ 9...11 => "lots of", //inclusive range //give a name to this range, call this range with this name 
        _ if (x%2 == 0) => "some",
        _ => "a few",
    }
}

fn pattern_matching(){

    for x in 0..13{
        println!("{}: I have {} oranges",x,pm_how_many(x));
    }
    
    let point=(3,4);
    match point
    {
        (0,0)=>println!("origin"),
        (0,y)=>println!("y axis, {}",y),
        (x,0)=>println!("x axis, {}",x), //If we want we can declare x as reference and mutable with "(ref mut x,0)=>..."
        (_,y)=>println!("(?,{})",y) //don't care x
    }


    //We can use it ".." instead of "_"(dont care) in enum cases.
    enum Color{
        RgbColor(u8,u8,u8), //tuple
        CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, //struct
    }
    let c:Color = Color::CmykColor{cyan:0, magenta:128, yellow:0, black:255};
    match c
    {
        //Color::RgbColor(0,0,0) | Color::CmykColor{cyan:_ ,magenta:_,yellow:_,black:255} => println!("Black"),
        Color::RgbColor(0,0,0) | Color::CmykColor{black:255,..} => println!("Black"), //Same before!
        _ => ()
    }

}

// Option<T,V>
struct generic_Point<T,V>
{
    x:T,
    y:V
}

struct generic_Line<T,V>
{
    start: generic_Point<T,V>,
    end: generic_Point<T,V>
}

fn generics(){
    let a:generic_Point<f64,f64> = generic_Point{ x:0.0,y:4f64 }; //can be also generic_Point<u16,i32>.
    let b = generic_Point{ x:1.2,y:3.4};

    let myline=generic_Line{ start: a, end:b };

}

fn main(){
    //pattern_matching();
    generics();
}