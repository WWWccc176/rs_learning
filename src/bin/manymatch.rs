fn main() {
    let v = Some(0u8);
    match v {
        Some(1) => println!("one"),
        _ => (),
    }
    if let Some(1) = v {
        println!("one");
    }
}
