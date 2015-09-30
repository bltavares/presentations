///Converts the string, char by char, into uppercase
///
///# Examples
///
///```
///let example = String::from("Hello World!");
///assert_eq!(example::upper(&example), "HELLO WORLD!");
///```
pub fn upper(s : &String) -> String {
    let uppers = s
        .chars()
        .flat_map(|x|
          x.to_uppercase()
         );
    uppers.collect()
}

