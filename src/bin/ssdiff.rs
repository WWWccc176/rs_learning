fn main() {
    let hd: u64 = 100;

    println!("{}", ssd(hd));
}

fn ssd(x: u64) -> u64 {
    let ssqu: u64 = (1..=x).map(|i| i * i).sum();
    let squs: u64 = (1..=x).sum();
    let diff: u64 = squs * squs - ssqu;
    diff
}
