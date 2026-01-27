fn main() {
    let bound = 600851475143;
    println!("{}", largest_prime_num(bound));
}

fn largest_prime_num(mut x: u64) -> u64 {
    let mut factor = 2;
    while factor * factor <= x {
        if x.is_multiple_of(factor) {
            x /= factor;
        } else {
            factor += 1;
        }
    }
    x
}
