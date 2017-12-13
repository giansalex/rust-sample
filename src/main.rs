extern crate regex;
extern crate rand;
extern crate requests;
extern crate handlebars;
#[macro_use]
extern crate serde_json;

mod regex_utils;
mod request_utils;
mod handlebar_utils;

fn main() {
    regex_utils::regex_work();
    request_utils::run();
    handlebar_utils::run();

    let tuple = rand::random::<(f64, char)>();
    println!("{:?}", tuple);

    println!("Hello, world!");
}