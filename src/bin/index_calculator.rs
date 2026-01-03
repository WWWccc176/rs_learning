fn main() {
    let basis: i16 = 9;
    let lar_num: i128 = 9832645832946582346583384562;

    let index: i128 = lar_num.ilog(basis as i128) as i128;
    println!(
        "Your large number lies in {basis}^{index} and {basis}^{}",
        index + 1
    )
}
