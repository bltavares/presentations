enum Flower {
    Rose,
    Violet,
}

fn color(flower: Flower) -> String {
    match flower {
        Flower::Rose => "red".to_string(),
        Flower::Violet => "blue".to_string(),
    }
}

fn main() {
    let flower = Flower::Rose;

    let inspect_color = color(flower);
    println!("The color is: {}", inspect_color);

    let inspect_color_2 = color(flower);
    println!("The color is: {}", inspect_color_2);
}
