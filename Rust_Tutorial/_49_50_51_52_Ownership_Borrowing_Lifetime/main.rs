#![allow(dead_code)]
#![allow(unused_variables)]

//1-OWNERSHIP
fn ownership(){
    let v=vec![1,2,3]; //v owns the memory which is stored in vector
    //let v2=v;//v2 is owner now.
    //println!("{:?}",v); //Gives Error, because v is already moved

    let foo = |v:Vec<i32>| ();
    //foo(v);
    //println!("{:?}",v); //It gives Error because v is already moved

    let t=1; //i32 Copy
    let y=t; //Only copying a variable
    println!("t={}",t); //It is OK

    let u=Box::new(1); //i32 
    let u2=u;
    //println!("u={}",*u); //It gives Error because u is already moved

    let print_vector =  |x:Vec<i32>| -> Vec<i32>
    {
        println!("{:?}",x);
        x
    };

    let vv=print_vector(v);
    println!("{}",vv[0]); //It is OK because it returns again and we take it.
   //println!("{}",v[0]); //It gives Error
}


//2-BORROWING
fn borrowing(){
    let v=vec![1,2,3];
    let print_vector =  |x:&Vec<i32>| 
    {
        println!("{:?}",x);
        println!("x[0]={}",x[0]);
    };

    print_vector(&v);
    println!("{:?}",v);//Ok
    println!("v[0]={}",v[0]); //Ok
  
    let mut a=40;
    let c= &a; //immutable reference
    let g= &a; //immutable reference
  
    println!("a={}",a);//ok
    println!("g={}",g);//ok
    println!("c={}",c);//ok
    println!("a={}",a);//ok

    {
    let b=&mut a; //mutable reference
    *b += 2; 
    //println!("a={}",a); //gives Error because it is borrowed to b now! Because b is used later.
    //a=45; //gives Error because it is borrowed to b now! Because b is used later.
    println!("b={}",b); //If we dont use it there, we can use the previous lines for a.
    }
    println!("a={}",a);
    a=45;
    println!("a={}",a);
    //println!("a={}",c);//Gives Error, it loses immutable reference after mutable reference

    let mut z=vec![3,2,1];
    for i in &z
    {
        println!("i={}",i);
        //z.push(5); //it gives Error, when we iterating the vector, it doesnot allow us to change vector inside.
    }
   
}    

//3-LIFETIME
struct Person{
    name:String
}
impl Person{
    fn get_ref_name(&self) -> &String{ //Compiler see this function as below
    //fn get_ref_name<'a>(&'a self) -> &'a String{ //Real function declaration of previous one //Lifetime Illusion
        &self.name
    }
}
struct Company<'z>
{
    name:String,
    ceo:&'z Person //If we only write &Person, compiler gives error when using later
}
fn lifetime(){
    let boss = Person {name:String::from("ElonMusk")};
    let tesla =Company{name:String::from("Tesla"),ceo: &boss};

    let mut z:&String;
    {
        let p=Person{name:String::from("John")};
        z=p.get_ref_name();
    }
    //println!("{}",z); //It gives error because p doesnot live now due to real function declaration above
}


//4-LIFETIME IN STRUCTURE IMPLEMENTATION
struct Person2<'a>
{
    name:&'a str
}

//impl<'b> Person2<'b>{ //It is OK
impl<'a> Person2<'a>{ //Convention: struct and impl have the same key for lifetime.
    fn talk(&self){
        println!("Hi my name is {}",self.name);
    }
}

fn lifetimeInStructureImp(){
    let person=Person2{name: "Dmitri"};
    person.talk();
}

fn main(){
    //ownership();
    //borrowing();
    //lifetime();
    lifetimeInStructureImp();
}