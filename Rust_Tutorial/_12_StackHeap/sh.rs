#![allow(dead_code)]
use std::mem;

struct Point
{
    x:f64,
    y:f64
}

fn origin() -> Point // "->" shows the return type.
{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap(){
    
    let p1 = origin();
    let p2 = Box::new(origin()); //To create dynamic variable, use Box::new(...)

    //mem::size_of_val shows the size in stack
    println!("p1 takes up {} bytes on the stack",mem::size_of_val(&p1)); //16 bytes due to 2 * f64(64bit=8 bytes)=16 bytes
    println!("p2 takes up {} bytes on the stack",mem::size_of_val(&p2)); //It stores memory adress of variable,
                                                                        // so the result is the same with the size of any address(Our computer is 64 bit=8byte).
                                                                        //result=8bytes(for stack). For heap, result will become 16 bytes. 
    let p3= *p2; //To unbox it
    println!("{}",p3.x);
}