fn main() {
    let max = 10001;
    let result: u64 = (2..)
        .filter(|&x| is_prime(x))
        .nth(max - 1)
        .expect("Should find a prime");
    println!("The result is: {}.", result);
}

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num.is_multiple_of(2) {
        return false;
    }

    let limit = (num as f64).sqrt() as u64;

    for i in (3..=limit).step_by(2) {
        if num.is_multiple_of(i) {
            return false;
        }
    }
    true
}
