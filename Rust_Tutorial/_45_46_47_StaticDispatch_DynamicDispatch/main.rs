#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::mem;

//1-STATIC DISPATCH

trait Printable
{
    fn format(&self)->String;
}
impl Printable for i32{
    fn format(&self)->String{
        format!("i32: {}",*self)
    }
}
impl Printable for String{
    fn format(&self)->String{
        format!("String: {}",*self)
    }
}

fn SD_print_it<T:Printable>(z:T){
    println!("{}",z.format());
} //monomorphisation
//Previous function returns to the below function at compile time when we use with string argument 
//fn SD_print_it(z:String){...} 

fn staticDispatch(){
    let a=123;
    let b="hello".to_string();

    println!("{}",a.format());
    println!("{}",b.format());

    SD_print_it(a);
    SD_print_it(b);
}

//2-DYNAMIC DISPATCH
//3-WHEN SHOULD WE USE DYNAMIC DISPATCH
fn DD_print_it(z:&Printable){
    println!("{}",z.format());
}

struct Circle{ radius:f64 }
struct Square { side: f64 }

trait Shape{
    fn area(&self)->f64;
}
impl Shape for Square{
    fn area(&self)->f64{
        self.side*self.side
    }
}
impl Shape for Circle{
    fn area(&self)->f64{
        self.radius*self.radius*std::f64::consts::PI
    }
}

fn dynamicDispatch(){
    let a=123;
    let b="hello".to_string();

    DD_print_it(&a);
    DD_print_it(&b);

    let shapes:[&Shape;4]= [&Circle{radius:1.0},&Square{side:3.0},&Circle{radius:2.0},&Square{side:4.0}];

    for (i,shape) in shapes.iter().enumerate(){
        println!("Shape #{} has area {}",i,shape.area());
    }
}






fn main(){
    //staticDispatch();
    dynamicDispatch();
}