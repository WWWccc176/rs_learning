fn main() {
    let mut s = String::from("hello world");
    let fir_word = first_word(&s);

    println!("{}", fir_word);
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
