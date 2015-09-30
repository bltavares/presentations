extern crate example;

use example::Upper;

fn main() {
    let example = String::from("Hello World!"); //A big source of confusion!
    println!("Initial input: {}, Modified: {}", example, example.upper());

    let another = example.chars().collect::<Vec<char>>();
    println!("Initial input: {}, Modified: {}", example, another.upper());
}
