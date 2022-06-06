fn if_statement(){

    let temp=15;
    if temp>30 
    {
        println!("high");
    }
    else if temp<10
    {
        println!("low");
    }
    else 
    {
        println!("OK");
    }

    let day = if temp>20 {"sunny"} else {"cloudy"};
    println!("today is {}",day);

    println!("it is {}", if temp>20 {"high"} else if temp<10 {"low"} else {"OK"});

    println!("it is {}", 
        if temp>20{ if temp>30 {"very high"} else {"high"} } 
        else if temp<10 {"cold"} 
        else {"OK"});
}

fn while_and_loop(){

    let mut x=1;
    let mut y=1;

    while x<1000
    {
        x*=2;
        if x==64 {continue;} 
        println!("x= {}",x);
    }

    loop{ // while true
        y*=2;
        println!("y= {}",y);
        if y== 1<<10 {break;} 
    }
}

fn for_loop(){

    for x in 1..11 // from 1 to 10
    {
        println!("x= {}",x); 
    }
    for (pos,y) in (30..41).enumerate() //to take both index and object in range
    {
        println!("{} : {}",pos,y);
    }
}

fn match_statement(){
    let country_code = 90;

    let country = match country_code{
        44 => "UK",
        90 => "TURKIYE",
        7 => "Russia",
        1..=1000 => "unknown", // =inclusive range, 1 to 1000 included 
        _ => "invalid"
    };

    println!("the country with code {} is {}",country_code,country);
}


fn main() {
    //if_statement();
    //while_and_loop();
    //for_loop();
    match_statement();
}