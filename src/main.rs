extern crate regex;
extern crate rand;
extern crate requests;

mod regex_utils;
mod request_utils;

fn main() {
    regex_utils::regex_work();
    request_utils::run();

    let tuple = rand::random::<(f64, char)>();
    println!("{:?}", tuple);

    println!("Hello, world!");
}