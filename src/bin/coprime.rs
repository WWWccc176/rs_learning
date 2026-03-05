fn main() {
    let a: u64 = 61251;
    let b: u64 = 3;
    let re = coprime(a, b);
    println!("{}", re);
}

fn coprime(x: u64, y: u64) -> bool {
    if x == y && x != 1 {
        return false;
    }

    if x.is_multiple_of(2) && y.is_multiple_of(2) {
        return false;
    }

    gcd(x, y) == 1
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut a = x;
    let mut b = y;
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
