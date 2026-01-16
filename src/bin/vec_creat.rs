fn main() {
    let a = [1, 2, 3, 4];
    let b = vec![1, 2, 3, 4];
    let c = Vec::from(a);

    println!("{:?}", b);
    println!("{:?}", c);
}
