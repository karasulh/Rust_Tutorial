use std::mem;

fn arrays(){
    let mut a:[i32;5]=[1,2,3,4,5]; //Type of elements are i32, Number of elements=5

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0]=321;
    println!("a[0]={}",a[0]);
    println!("{:?}",a); //Print all elements of a

    if a == [321,2,3,4,5]
    {
        println!("ok");
    }
    let b=[1u16;10]; //1 which is 16 bit //b=[1;10]=>1 which is 32 bit //Initialize 10 elements with 1. //b.len()=10
    for i in 0..b.len()
    {
        println!("{}",b[i]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b)); //20 bytes if 16 bit, 40 bytes if 32 bit

    let matrix:[[f32;3];2] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 3.0]
    ];

    println!("{:?}",matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i==j {
                println!("matrix[{}][{}]={}",i,j,matrix[i][j]);
            }
        }
    }
}


fn use_slice(slice: &mut [i32]) //to make slice mutable, add mut keyword
{
    println!("first element of slice={},len={}",slice[0],slice.len());
    slice[0]=1453;
}

fn slices()
{
    let mut data =[1,2,3,4,5];

    use_slice(&mut data[1..4]); //give it with &mut
    //use_slice(&mut data);

    println!("{:?}",data);
}

fn tuples_sum_and_product(x:i32, y:i32)->(i32,i32)
{
    (x+y,x*y)
}

fn tuples(){
    let x=3;
    let y=4;
    let sp=tuples_sum_and_product(x,y);

    println!("sp= {:?}",sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}",x,y,sp.0,sp.1);

    //destructuring
    let (a,b) =sp;
    println!("a ={}, b={}",a,b);

    let sp2=tuples_sum_and_product(4,7);
    let combined=(sp,sp2);
    println!("{:?}",combined);
    println!("last elem={}",(combined.1).1);

    let ((c,d),(e,f))=combined;
    println!("c={}, f={}",c,f);
    let combined2=((c,d),(e,f));
    println!("last elem={}",(combined2.1).1);

    let foo=(true,42.0,-1i8);
    println!("{:?}",foo);

    let meaning=(42,);
    println!("{:?}",meaning);
}


fn main(){
    //arrays();
    //slices();
    tuples();
}