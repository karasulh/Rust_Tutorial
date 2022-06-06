extern crate phrases;

use phrases::greetings::turkish;

fn main(){
    println!("English: {}, {}",phrases::greetings::english::hello(),phrases::greetings::english::goodbye());
    println!("Turkish: {}, {}",turkish::hello(),turkish::goodbye());
}

// #[test]
// fn english_greeting_correct() {
//     assert_eq!("hello",greetings::english::hello());
// }