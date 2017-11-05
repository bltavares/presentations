extern crate reqwest;

use std::io::Read;

fn main() {
    let mut response = reqwest::get("https://httpbin.org/get")
                                .expect("Could not connect");
    println!("Resposta: {:?}", response);

    let mut content = String::new();
    response.read_to_string(&mut content);
    println!("Conteudo do site: {}", content);
}
