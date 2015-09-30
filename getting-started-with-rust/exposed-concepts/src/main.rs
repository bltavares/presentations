extern crate example;

fn main() {
    let example = String::from("Hello World!"); //A big source of confusion!
    let transformed = example::upper(&example);
    println!("Initial input: {}, Modified: {:?}", example, transformed);

    //Move semanatics and ownership
    let steal_ownership = &transformed;
    println!("Initial input: {}, Modified: {:?}", example, transformed);

    //Heap allocation
    //let this_lives_out_of_the_stack = Box::new(transformed);
}
