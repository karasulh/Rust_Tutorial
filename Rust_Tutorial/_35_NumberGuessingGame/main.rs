#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::io::stdin;
use rand::Rng;
extern crate rand;

fn main(){
    let number = rand::thread_rng().gen_range(1,101); //random number generator

    loop{
        println!("Enter your guess: ");
        let mut buffer=String::new();
        match stdin().read_line(&mut buffer){
            Ok(_)=>{ //Show the input type as i64 when parsing or in the variable    
                //let parsed:i64 =buffer.trim_end().parse();
                let parsed=buffer.trim_end().parse::<i64>(); //seperate input from the line break
                
                match parsed{
                    Ok(guess) => {
                        if guess<1 || guess>100 {println!("out of range");}
                        else if guess<number {println!("guess is lower");}
                        else if guess>number {println!("guess is higher");}
                        else {println!("Correct!"); 
                            break; }
                    }
                    Err(e) => {
                        println!("Couldnot read input, {}, Try again!",e);
                    }
                }
            },
            Err(_)=>{continue;}
        }
    }
}