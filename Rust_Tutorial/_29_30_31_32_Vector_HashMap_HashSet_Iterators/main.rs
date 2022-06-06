#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::collections::HashMap; //for hashmap

use std::collections::HashSet; //for hashset
use std::thread;
use std::time;

//VECTORS
fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a={:?}",a);
    a.push(4);
    println!("a={:?}",a);
    
    let idx:usize = 0;
    a[idx]=312;
    println!("a[0]={}",a[idx]);

    //Option
    match a.get(6) //Returns a "Option" //There is no element at 6 index
    {
        Some(x) => println!("a[3]={}",x),
        None => println!("error, no such element")
    }

    for x in &a { //Iterating in vector //Give it as reference, because iterating with loop on vector, moves it and not to lose it use reference. 
        println!("{}",x);
    }
    
    a.push(77);
    a.push(88);
    a.push(99);
    println!("a={:?}",a);

    let last_element_Option = a.pop(); //Pop returns a type of "Option".
    println!("Last element is {:?}, a={:?}",last_element_Option,a);

    if let Some(x) = a.pop()
    {
        println!("Last element with if-let: {}",x);
    }
    while let Some(x) = a.pop()//Iterate all elements by popping
    {
        println!("Elements with while-let: {}",x);
    }
}


//HASHMAPS
fn hashMaps(){

    let mut shapes = HashMap::new();
    
    shapes.insert(String::from("triangle"),3);
    shapes.insert(String::from("square"),4);

    for (key,value) in &shapes{
        println!("{}:{}",key,value);
    }

    shapes.insert("square".into(),5); //it overwrites the previous
    println!("{:?}",shapes);

    //".entry" checks whether "circle" exists before.
    //If not, add it. If yes, don't change anything.
    shapes.entry("circle".into()).or_insert(1);

    {
        let actual = shapes.entry("circle".into()).or_insert(2); //this returns a reference to element
        *actual=0;//make it 0, inside or outside scope, it becames 0 due to referenceness.
    }
    
    println!("{:?}",shapes);
    println!("{}",shapes["square"]);
}


//HASHSETS
fn hashSets(){

    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}",greeks);

    let added_vega = greeks.insert("vega"); //return boolean
    if added_vega{//if "vega" already exist , "else" is working.
        println!("we added vega!");
    }

    if !greeks.contains("kappa"){ //return boolean
        println!("we dont have kappa");
    }

    let removed=greeks.remove("delta");
    if removed{
        println!("we removed delta");
    }
    println!("{:?}",greeks);

    let _1_5: HashSet<_> = (1..5).collect();
    let _6_10: HashSet<_> = (6..10).collect();
    let _1_10: HashSet<_> = (1..10).collect();
    let _2_8: HashSet<_> = (2..8).collect();

    //subset
    println!("is {:?} a subset of {:?}? {}",_2_8,_1_10,_2_8.is_subset(&_1_10));
    //disjoint=no common elements
    println!("{:?} and {:?} disjoint? {}",_1_5,_6_10,_1_5.is_disjoint(&_6_10));
    //union
    println!("items in either {:?} and {:?} are {:?}",_2_8,_6_10,_2_8.union(&_6_10));
    //intersection =>> _1_10.intersection(&_2_8)
    //difference =>> _1_10.difference(&_2_8)
    //symmetric difference=union-intersection =>> _1_10.symmetric_difference(&_2_8)
}


//ITERATORS
fn iterators(){
    /*  
    let vec=vec![3,2,1]; //immutable 
    for x in &vec 
    {   //For the println, we can also write only "x", because Rust can understand whether it should be dereferenced or not.
        println!("{}",*x); //It adjusts it itself.
    }
    for x in vec.iter() //immutable
    {   
        println!("we got {}",x);//Same rules are valid in also this case. x or *x
    }
    */
    
    let mut vec=vec![3,2,1]; //mutable
    for x in vec.iter_mut() //mutable iterator
    {
        *x += 2;
    }
    println!("{:?}",vec);

    for x in vec.iter().rev(){
        println!("in reverse: {}",x);
    }
    
    let mut vec1= vec![3,2,1];
    let mut vec2= vec![1,2,3];
    vec2.extend(vec1);
    println!("{:?}",vec2);
    //println!("{:?}",vec1); //gives error, because vec1 was moved due to into_iter() which is in extend().
    
    
}

fn main(){
    //vectors();
    //hashMaps();    
    //hashSets();   
    iterators();
}