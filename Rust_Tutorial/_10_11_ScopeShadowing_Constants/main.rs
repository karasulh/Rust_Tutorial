use std::mem;

const MEANING_OF_LIFE: u8 = 42; //const: No fixed address. Type of const must be given.
static z:i32 = 123; //have fixed address
static mut y:i32 = 123; // should be used with unsafe block if static is mutable



fn main(){
   
    println!("z={}",z);  

    unsafe{
        println!("y={}",y); 
    } 
    
    scope_and_shadowing();
}



fn scope_and_shadowing(){
    
    let a=123;
    {
        let b=456;
        let a=789;
        println!("inside, b={}",b);
        println!("inside, a={}",a);
    }
    println!("a={}",a);
   //println!("outside, b ={}",b); //gives an error.
}