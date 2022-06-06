#![allow(dead_code)]
#![allow(unused_variables)]
//1-REFERENCE-COUNTED VARIABLES(RC)
use std::rc::Rc;//1
use std::thread;//2
use std::sync::{Arc,Mutex};//2,3


struct Person{
    name: Rc<String>,
    num:i32
}
impl Person{
    fn new(name:Rc<String>,num:i32)->Person{
        Person{name:name,num:num}
    }
    fn greet(&self){
        println!("Hi, my name is {}",self.name);
    }
}
fn rc_demo()
{
    let name=Rc::new("John".to_string()); //Rc::new(String)     
    let num=5;
    println!("Name={}, name has {} strong pointers",name,Rc::strong_count(&name));
    {
    let person=Person::new(name.clone(),num); //give it with clone  
    println!("Name={}, name has {} strong pointers",name,Rc::strong_count(&name));
    person.greet();
    }
    println!("Name={}, name has {} strong pointers",name,Rc::strong_count(&name));    
    //println!("Name:{}",name);//it gives error,because normal "name" is moved with Person::new function, before.
    println!("Name:{}",name);//Ok, because we use Rc, we use clone, there is no "move"
    println!("Num:{}",num); //OK
}

//2-ATOMIC REFERENCE-COUNTED VARIABLES (ARC)

struct Person2{
    name: Arc<String>,
}
impl Person2{
    fn new(name:Arc<String>)->Person2{
        Person2{name:name}
    }
    fn greet(&self){
        println!("Hi, my name is {}",self.name);
    }
}
fn arc_demo(){
    let name=Arc::new("John".to_string());
    let person=Person2::new(name.clone());

    let t=thread::spawn(move || { //create a thread from its template
       person.greet(); 
    });
    println!("Name={}",name);//OK.
    t.join().unwrap();//wait thread to finish
}


//3-USING a MUTEX for THREAD-SAFE MUTABILITY
struct Person3{
    name: Arc<String>,
    state: Arc<Mutex<String>>
}
impl Person3{
    fn new(name:Arc<String>,state:Arc<Mutex<String>>)->Person3{
        Person3{name:name,state:state}
    }
    fn greet(&self){
        
        let mut state=self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        
        println!("Hi, my name is {} and I am {}",self.name,state.as_str());
    }
}
fn mutex_arc_demo(){
    let name=Arc::new("John".to_string());
    let state= Arc::new(Mutex::new("bored".to_string()));
    let person=Person3::new(name.clone(),state.clone());

    let t=thread::spawn(move || { //create a thread from its template
       person.greet(); 
    });
    println!("Name={},State={}",name,state.lock().unwrap().as_str());//OK.
    t.join().unwrap();//wait thread to finish
}


fn main(){
    //rc_demo();
    //arc_demo();
    mutex_arc_demo();
}