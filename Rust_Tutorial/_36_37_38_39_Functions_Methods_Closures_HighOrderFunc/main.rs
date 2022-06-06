#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

//1-FUNCTIONS
fn func_print_value(x:i32){
    println!("value= {}",x);
}
fn func_increase(x: &mut i32){ //reference
    *x +=1; //dereference
}
fn func_product(x:i32,y:i32)->i32
{
    x*y
}

fn functions(){
    func_print_value(33);

    let mut z=1;
    func_increase(&mut z); //change original variable
    println!("z={}",z);

    let a=3;
    let b=5;
    let p=func_product(a,b);
    println!("{}*{}={}",a,b,p);

    let mut x=0;
    let y=&mut x;
    //let z=&mut x; //ERROR, x is already borrowed mutably
    *y=1;
    println!("{}",x); //x=1
}


//2-METHODS
struct  Point
{
    x:f64,
    y:f64
}
struct Line
{
    start: Point,
    end: Point
}
impl Line //methods for struct Line
{
    fn len(&self)->f64{
        let dx=self.start.x-self.end.x;
        let dy=self.start.y-self.end.y;
        (dx*dx+dy*dy).sqrt()     
    }
}

fn methods(){
    let p = Point{ x: 3.0, y:4.0};
    let p2 = Point{ x:5.0, y: 10.0};
    let myLine = Line {start:p , end:p2};
    println!("length={}",myLine.len());
}


//CLOSURE
fn clo_say_hello(){ println!("Hello");}

fn closures(){
    let sh = clo_say_hello;//Variable can store function
    sh();

    let plus_one = |x:i32| -> i32 {x+1};
    let a=6;
    println!("{}+1={}",a,plus_one(a));

    let mut two=2;
    {
        let plus_two = |x|
        {
            let mut z=x;
            z+=two;
            z
        };
        println!("{}+2={}",3,plus_two(3));
    }
    let borrow_two= &mut two;
    println!("{}",borrow_two);

    //T:by value
    //T&:as reference
    //&mut 

    let plus_three= |x: &mut i32| *x+=3;//give it as referenced

    let mut f=12;
    plus_three(&mut f);
    println!("f={}",f);
}

//HIGH ORDER FUNCTION

//functions that take functions
//f(g) {let x=g;} 

//functions that return function
//generators
//f()->g


fn HO_is_even(x:u32)->bool{
    x%2==0
}

fn HO_greater_than(limit:u32) -> impl Fn(u32) -> bool  //function which return function, must take these signatures
{
    move |y| y>limit // put a move keyword to extend lifetime of y
}

fn highOrderFunctions(){

    let limit =500;
    let mut sum=0;
    let above_limit = |y:u32| y>limit; //Closure
    let above_limit2 = HO_greater_than(limit); //For the case which "function returns function"

    for i in 0..{
        let isq = i*i;

        //if isq>limit {break;} //Normal
        //if above_limit(isq) {break;} //Closure
        if above_limit2(isq) {break;} //For the case which "function returns function"

        else if HO_is_even(isq){ sum += isq; }
    }
    println!("loop sum={}",sum);

    //map:takes a value and transforms to other value
    //take_while: controls the loop condition
    //filter: to filter,check condition
    //fold: pairwise operation like accumulator
    let sum2=(0..) 
        .map(|x| x*x) 
        .take_while(|&x| x<limit) 
        .filter(|x:&u32| HO_is_even(*x)) 
        .fold(0,|sum,x| sum+x);

    println!("highOrderFunc sum={}",sum2);

}


fn main(){
    //functions();
    //methods();
    //closures();
    highOrderFunctions();


}