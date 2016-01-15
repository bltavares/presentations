enum Flower {
    Rose,
    Violet,
}

#[derive(Debug)]
struct Color {
    red: u8,
    blue: u8,
    green: u8,
}

fn color(flower: Flower) -> Color {
    match flower {
        Flower::Rose => Color { red: 255, blue: 0, green: 0 },
        Flower::Violet => Color { blue: 255, red: 0, green: 0 },
    }
}

fn main() {
    let flower = Flower::Rose;

    let inspect_color = color(flower);
    println!("The color is: {:?}", inspect_color);

    let inspect_color_2 = color(flower);
    println!("The color is: {:?}", inspect_color_2);
}
