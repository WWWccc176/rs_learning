fn main() {
    let num2: &mut i32 = &mut 5;

    *num2 += 1;
    let num1 = num2;

    println!("Number plus two is: {}", num1);
}
