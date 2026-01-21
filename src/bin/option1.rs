fn main() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let abs: Option<i32> = Option::None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", abs);
}
#[derive(Debug)] //这行可以让编译器把代码打出来。由于性能考虑编译器不支持打印，要用derive(debug)告诉它。
enum Option<T> {
    None,
    Some(T),
}
