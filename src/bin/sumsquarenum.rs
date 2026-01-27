fn main() {
    let ans = sum_oddsquare_num(948000);
    println!("{ans}");
}

fn sum_oddsquare_num(x: i128) -> i128 {
    (1..=x).map(|i| i * i).filter(|i| i % 2 != 0).sum() //格式一定要详记！
}
