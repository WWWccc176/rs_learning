fn main() {
    let mut s = String::from("Hello! ");

    push(&mut s);
    println!("{}", s);
}

fn push(str: &mut String) {
    //很方便，比直接操作简洁
    str.push_str("World!");
}
