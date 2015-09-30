pub trait Upper {
    fn upper(&self) -> String;
}

impl Upper for String {
    fn upper(&self) -> String {
        let uppers = self
            .chars()
            .flat_map(|x|
              x.to_uppercase()
             );
        uppers.collect()
    }
}

impl Upper for Vec<char> {
    fn upper(&self) -> String {
        let uppers = self.iter().flat_map(|x| x.to_uppercase());
        uppers.collect()
    }
}

struct Banana {
    id: i32,
}

impl Upper for Banana {
    fn upper(&self) -> String {
        String::from("banana")
    }
}
