fn main() {
    let a: f32 = 0.1;
    let b: f64 = 0.1;
    let aa: f32 = a + a + a + a + a + a + a + a + a + a;
    let bb: f64 = b + b + b + b + b + b + b + b + b + b;

    println!("{:.16}", aa);
    println!("{:.16}", bb);
}
