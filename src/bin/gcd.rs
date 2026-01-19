fn main() {
    let x: i32 = 654651;
    let y: i32 = 512;
    let g_c_d = gcd(x, y);

    println!("gcd of x and y is: {}", g_c_d);
}

fn gcd(mut x: i32, mut y: i32) -> i32 {
    assert!(x != 0 && y != 0);
    while x != 0 {
        if x < y {
            let t = x;
            y = t;
            x = y;
        }
        x = x % y
    }
    x
}
