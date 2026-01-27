fn main() {
    let bound = 1000;

    println!("Result is: {}", sum_35_mul(bound));
}

fn sum_35_mul(x: u64) -> u64 {
    (3..x).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}
