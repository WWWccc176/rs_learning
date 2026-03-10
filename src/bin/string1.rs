fn main() {
    let mut s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    s.push('呀');
    s.insert(6, ' ');
    say_hello(&s);

    let new_s = s.replacen(' ', ".", 1);
    say_hello(&new_s);

    for c in new_s.chars() {
        println!("{}", c)
    }
}

fn say_hello(s: &str) {
    println!("{}", s);
}
