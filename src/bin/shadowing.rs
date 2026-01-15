fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("内部作用域的 x: {}", x);
    }
    println!("外部作用域的 x: {}", x);
}
