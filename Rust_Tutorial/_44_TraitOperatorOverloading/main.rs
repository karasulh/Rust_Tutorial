#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::ops::{Add,AddAssign,Neg};

//#[derive(Debug,PartialEq,Eq,Ord,PartialOrd)] //We can use operator by deriving 
#[derive(Debug)]
struct Complex<T>
{
    re:T,
    im:T
}
impl<T> Complex<T>{
    fn new(re:T,im:T)->Complex<T>{
        Complex::<T> {re,im}
    }
} 

//To make generic, instead of i32, write T,
//and also specify T supports "addition(+)" with "where" keyword 
//to say output is also T for "+" like "self.re + rhs.re"

//impl Add for Complex<i32> {
//  type Output = Complex<i32>;
impl<T> Add for Complex<T>
    where T:Add<Output = T>
{
    type Output = Complex<T>;
    fn add(self,rhs:Self) -> Self::Output{
        Complex{
            re: self.re+rhs.re,
            im: self.im+rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T:AddAssign<T>
{
    fn add_assign(&mut self,rhs:Self){
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
    where T:Neg<Output = T>
{
    type Output=Complex<T>;
    fn neg(self) -> Self::Output{
        Complex{
            re:-self.re,
            im:-self.im,
        }
    }
}

//partial eq
//full eq: x=x
//NAN=not a number 0/0 inf/inf
//NAN==NAN => false

impl<T> PartialEq for Complex<T>
    where T:PartialEq
{
    fn eq(&self,rhs:&Self)->bool{//Equal Operator Overloading
        self.re == rhs.re && self.im == rhs.im
    }
    fn ne(&self, rhs:&Self)->bool{//Not Equal Operator Overloading
        self.re != rhs.re || self.im != rhs.im
    } 
}

//No need to implement full equality trait, because it uses PartialEquality trait
//impl<T:Eq> Eq for Complex<T> where T:Eq{} //for full equality

fn main(){

    let mut a=Complex::new(1.0,2.0);
    let mut b=Complex::new(3.0,4.0);
    //println!("{:?}",a+b);
    //a+=b;
    println!("{:?}",a==a);
}