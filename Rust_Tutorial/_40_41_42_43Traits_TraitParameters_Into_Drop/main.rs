#![allow(dead_code)]
#![allow(unused_imports)]
use std::fmt::Debug; //added for trait parameters subject

//1-TRAIT
trait Animal 
{   
    fn create(name: &'static str)->Self; //Static function
    fn name(&self) -> &'static str; //Instance function
    fn talk(&self)                  //Instance function
    {
        println!("{} cannot talk",self.name());
    }
}
struct Human
{
    name: &'static str
}
impl Animal for Human
{   
    fn create(name:&'static str)->Human
    {
        Human{name:name}
    }
    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self)
    {
        println!("{} says hello",self.name());
    }
}

struct Cat
{
    name: &'static str
}
impl Animal for Cat
{   
    fn create(name:&'static str)->Cat
    {
        Cat{name:name}
    }
    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self)
    {
        println!("{} says miyav",self.name());
    }
}

trait Summable<T>
{
    fn sum(&self)->T;
}

impl Summable<i32> for Vec<i32> //Use it for generic Vec<i32> type
{
    fn sum(&self)->i32
    {
        let mut result:i32 =0;
        for x in self { result += *x; } //self=vector
        return result;
    }
}

fn traits(){

    //let h= Human{name:"John"}; //Normal instance
    //let h=Human::create("John"); 
    let h:Human= Animal::create("John"); //Factory instance
    h.talk();
    let k=Cat{name:"Misha"};
    k.talk();

    let a=vec![3,2,1];
    println!("sum={}",a.sum()); 
}


//2-TRAIT PARAMETERS

#[derive(Debug)]
struct Circle{
    radius:f64,
}
#[derive(Debug)]
struct Square{
    side:f64,
}

trait Shape{
    fn area(&self) -> f64;
}
impl Shape for Square{
    fn area(&self)->f64{
        self.side * self.side
    }
}
impl Shape for Circle{
    fn area(&self)->f64{
        self.radius*self.radius * std::f64::consts::PI
    }
}

//fn TP_print_info(shape: impl Shape + Debug)
//fn TP_print_info<T>(shape:T) where T:Shape+Debug
fn TP_print_info<T:Shape+Debug>(shape:T)
{   
    println!("{:?}",shape);
    println!("The area is {}",shape.area());
}
fn traitParameters(){
    let c=Circle{radius: 2.0};
    TP_print_info(c);
}


//3-INTO

struct Person{ name: String}
impl Person{
    
    //Previous/Normal Version
    /*
    fn new(name: &str)->Person{ //take argument as str
        Person{name:name.to_string()} //convert str to String
    }
    */
    
    //Version with Into Trait
    //fn new<S>(name:S)->Person where S:Into<String>  //Third way
    fn new<S: Into<String>>(name:S) -> Person //Second way
    {
        Person{ name: name.into()}
    }
}

fn IntoTraitFunc(){
    let John = Person::new("John");
    let name:String = "Jane".to_string();
    //let jane=Person::new(name.as_ref()); //For previous/Normal version
    let jane=Person::new(name);//For version with Into Trait
    //println!("{:?}",jane);//Can not be used without Debug trait for case with Into Trait
}


//DROP
struct Creature{
    name:String
}
impl Creature{
    fn new(name:&str)->Creature{
        println!("{} enters the game",name);
        Creature{name:name.into()}
    }
}
impl Drop for Creature{
    fn drop(&mut self){
        println!("{} is dead",self.name);
    }
}

fn DropTraitFunc(){

    let mut clever:Creature;
    {
        let goblin=Creature::new("Jeff");
        println!("game proceeds");
        clever=goblin;
        println!("end of scope");
    }
    // let goblin=Creature::new("Jeff");
    // println!("game proceeds");
    //goblin.drop(); //Error
    //drop(goblin); //OK
}

fn main(){
    //traits();
    //traitParameters();
    //IntoTraitFunc();
    DropTraitFunc();
}