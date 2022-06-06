#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]


fn strings(){
    //utf-8
    let s : &'static str = "hello there"; //&str=string slice
    //s="abc"; //gives error, Changing static string is not allowed.
    //let h=s[0]; //gives error, Reading char of static string is not allowed.

    for c in s.chars(){ //s.chars().rev()
        println!("{}",c);
        //c='s'; //gives error
    }
    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}",first_char);
    }


    //heap
    //String
    let mut letters = String::new();
    let mut a='a' as u8; //define ascii int value of 'a'
    while a <= ('z' as u8)
    {
        letters.push(a as char); //define char of 'a' 
        letters.push_str(",");
        a+=1;
    }
    println!("{:?}",letters);

    //&str <= String
    let u: &str = &letters;

    //concatenation
    //String+str
    //let z1= letters + "abc"; 
    let letters2="ggzytm".to_string(); //String
    let z = letters + &letters2; //String+String //think &letters2(&String) as string which is &str type
    //let z = letters + &letters; //NOT VALID 
    
    println!("{:?}",z);

    //String <= str
    let mut abc1= String::from("Hello");
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}",abc.replace("ello","goodbye"));
}


fn stringFormatting(){

    let name="Dmitri";
    let greeting = format!("hi,I'm {}, nice to  meet you",name);
    println!("{}",greeting);

    let hello ="hello";
    let rust ="rust";
    let hello_rust = format!("{}, {}!",hello,rust);
    println!("{}",hello_rust);

    let run="run";
    let forest="forest";
    let rfr= format!("{0},{1},{0}!",run,forest);
    println!("{}",rfr);

    let info= format!("the name's {last}, {first} {last}",first="James",last="Bond");
    println!("{}",info);

    let mixed = format!("{1} {} {0} {} {data}","alpha","beta",data="delta");
    println!("{}",mixed);

    //Gives Error! "gamma" is not used.
    //let mixed = format!("{1} {} {0} {} {data}","alpha","beta","gamma",data="delta"); 
    
}

fn main(){
    //strings();
    stringFormatting();
}