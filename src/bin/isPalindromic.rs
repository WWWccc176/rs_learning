fn main() {
    let max_3: u64 = 999;
    println!("The result is: {}.", palindrome_iter(max_3));
}

fn is_palindromic(mut x: u64) -> bool {
    let original = x;
    let mut reversed = 0;

    while x > 0 {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }

    original == reversed
}

//性能相比命令式的for loop，性能稍差。O(k) VS O(n^2)
fn palindrome_iter(a: u64) -> u64 {
    (100..=a)
        .flat_map(|i| (100..=i).map(move |j| i * j))
        .filter(|&p| is_palindromic(p))
        .max()
        .unwrap_or(0)
}
