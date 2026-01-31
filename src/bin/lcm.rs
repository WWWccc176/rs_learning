fn main() {
    let max: u64 = 20;
    println!("The result is: {}.", lcm(max));
}

fn lcm(a: u64) -> u64 {
    (2..=a)
        .filter(|&x| is_prime(x))
        .map(|p| {
            let mut max_pow = p;
            while max_pow * p <= a {
                max_pow *= p;
            }
            max_pow
        })
        .product()
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
