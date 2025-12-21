fn main() {
    let basis: i16 = 9;
    let lar_num: i128 = 83264583246582346583384562;

    let index: i128 = lar_num.ilog(basis as i128) as i128;
    println!(
        "The large number lies in {basis}^{index} and {basis}^{}",
        index + 1
    )
}
