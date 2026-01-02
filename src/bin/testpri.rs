fn main() {
    let a: u32 = 10;
    let b: u32 = 0;
    if !a.is_multiple_of(b) {
        println!("1")
    }
    println!("{}", a.is_multiple_of(2)); // 输出 false，程序不会崩溃
}
