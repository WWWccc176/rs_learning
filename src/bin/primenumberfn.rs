fn main() {
    let a = 4654646;
    println!("{}", is_prime_num(a))
}
fn is_prime_num(input: u64) -> bool {
    if input <= 1 {
        return false;
    }
    if input == 2 {
        return true;
    }
    if input.is_multiple_of(2) {
        return false;
    }
    let limit = (input as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if input.is_multiple_of(i) {
            return false;
        }
    }
    true
}
