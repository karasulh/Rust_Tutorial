#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    
    let a: u8 = 123; //u8=uint8 -> unsigned
    println!("a= {}",a); //immutable

    let mut b: i8 =0; //i8=int8 -> signed 
    println!("b= {} before",b); //mutable
    b=42;
    println!("b= {} after",b);

    let c = 1234556789; //i32 =32 bits = 4 bytes
    println!("c= {}, takes up {} bytes", c,mem::size_of_val(&c));

    //u8,u16,u32,u64,i8,i16,i32,i64

    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z); //write "usize" or not, does not care.
    println!("z = {}, takes up {} bytes, {}-bit OS", z,size_of_z,size_of_z*8); //1byte=8bit

    let d:char = 'x';
    println!("{} is a char, size ={} bytes",d,mem::size_of_val(&d));

    let e: f32 = 2.5; //The default is f64 for floating points
    println!("{}, size={} bytes",e,mem::size_of_val(&e));

    let g: bool = false;
    println!("{}, size={} bytes",g,mem::size_of_val(&g));
}
