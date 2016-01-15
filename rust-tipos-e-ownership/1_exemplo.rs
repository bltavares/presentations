fn color(flower: String) -> String {
    if flower == "Roses" {
        "red".to_string()
    } else if flower == "Violets" {
        "blue".to_string()
    } else {
        panic!("So far only Roses or Violets") 
    }
}

fn main() {
    let flower = "Roses".to_string();
    let inspect_color = color(flower);

    println!("The color is: {}", inspect_color);
}
