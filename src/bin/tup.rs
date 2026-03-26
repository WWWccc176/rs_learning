fn main() {
    let tup: (i64, u16, String) = (132564978, 46564, String::from("string"));

    let (a, b, c) = &tup;

    println!("{}{}{}", a, b, c);
    println!("{}", tup.1);
    println!("{:?}", calculate_length(tup.2));
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
