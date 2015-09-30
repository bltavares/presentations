extern crate example;

use example::upper;

fn main() {
    let example = String::from("Hello World!"); //A big source of confusion!
    println!("Initial input: {}, Modified: {}", example, upper(&example));
}
