fn main() {
    let s = String::from("Nice to meet you!\n"); //占位符长度为1.
    let len = len_cal(&s);
    println!("The length of s is {}.", len);
}

fn len_cal(s: &String) -> usize {
    s.len()
}
