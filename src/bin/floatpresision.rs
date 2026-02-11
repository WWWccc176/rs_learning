fn main() {
    let a: f32 = 0.1;
    let b: f64 = 0.1;
    let c: f32 = 1.;
    let d: f64 = 1.;
    let aa: f32 = a + a + a + a + a + a + a + a + a + a;
    let bb: f64 = b + b + b + b + b + b + b + b + b + b;
    let cc: f32 = c + c + c + c + c + c + c + c + c + c;
    let dd: f64 = d + d + d + d + d + d + d + d + d + d;
    let bbb:f64=b*10.;

    println!("{:.25}", aa);
    println!("{:.25}", bb);
    println!("{:.25}", cc);
    println!("{:.25}", dd);
    println!("{:.25}", bbb);
}
