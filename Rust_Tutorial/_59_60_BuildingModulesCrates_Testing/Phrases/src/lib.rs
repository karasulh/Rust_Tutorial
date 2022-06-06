pub mod greetings
{
    pub mod english;
    pub mod turkish{
        pub fn hello() -> String{"selam".to_string()}
        pub fn goodbye() -> String{"gulegule".to_string()}
    }
}

// #[test]
// #[should_panic]
// #[ignore]
// fn english_greeting_correct() {
//     assert_eq!("hellodsf",greetings::english::hello());
// }
