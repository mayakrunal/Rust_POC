fn main() {
    let mut s = String::from("Hello World!");
    // let hello = &s[..5];
    // let world = &s[6..];
    let first = first_word(&s);

    println!("{}", first);

    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}
