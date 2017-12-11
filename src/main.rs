extern crate regex;
extern crate rand;

mod regex_utils;


fn main() {
    regex_utils::regex_work();

    let tuple = rand::random::<(f64, char)>();
    println!("{:?}", tuple);

    println!("Hello, world!");
}
