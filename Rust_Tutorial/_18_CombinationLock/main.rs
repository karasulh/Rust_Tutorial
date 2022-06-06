/*This code is written about to show loop, enums and match staments. It gets strings from user and compare it with "code"="1234". 
According to comparison results, it changes its states.  */ 

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

//use rand::Rng;
use std::io::stdin;

enum State{
    Locked,
    Failed,
    Unlocked
}

fn main(){

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new(); //create empty string

    loop{
        match state{
            State::Locked => {
                let mut input= String::new();
                match stdin().read_line(&mut input){ //It returns Ok or Err. //It wants &mut variable.
                    Ok(_) => { // Put "_" if we dont care inside.
                        entry.push_str(&input.trim_end()); //it gets each user input, and put into "entry" 
                    } 
                    Err(_) => continue //{continue;}
                }
                if entry == code{
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry){ //code=1234 and entry=125, then entry is cleared, state is passing to Failed
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }    
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }

            
        }

    }



}