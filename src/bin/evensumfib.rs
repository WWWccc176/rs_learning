fn main() {
    let bound = 4000000;

    println!("Result is: {}", sum_even_feb(bound));
}

fn sum_even_feb(x: u64) -> u64 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 2;

    while b <= x {
        if b % 2 == 0 {
            sum += b
        }
        let next = a + b;
        a = b;
        b = next;
    }
    sum
}
