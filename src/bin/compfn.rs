use num::integer::sqrt;

fn main() {
    let a: i32 = 12;
    let b: i32 = 13;
    let e = add(modu(a, b), tri(a, b));

    println!("e = {e}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn modu(x: i32, y: i32) -> i32 {
    sqrt(add(x.pow(2), y.pow(2)))
}

fn tri(x: i32, y: i32) -> i32 {
    x.pow(y as u32)
}
