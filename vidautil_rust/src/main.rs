fn more_largo<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    println!("Vida Util Rust!!");

    let string1 = String::from("abcd");
    let string2 = "xysds";

    let result = more_largo(string1.as_str(), string2);
    println!("La cadena mas larga es:  {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    
}
