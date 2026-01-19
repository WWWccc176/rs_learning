fn main() {
    let value1 = value_in_unit(Currency::Fen);
    let value2 = value_in_unit(Currency::Jiao);
    let value3 = value_in_unit(Currency::Yuan);
    let value4 = value_in_unit(Currency::Bai);

    println!("{}", value1);
    println!("{}", value2);
    println!("{}", value3);
    println!("{}", value4);
}

enum Currency {
    Fen,
    Jiao,
    Yuan,
    Bai,
}

fn value_in_unit(currency: Currency) -> u16 {
    match currency {
        Currency::Fen => 1,
        Currency::Jiao => 10,
        Currency::Yuan => 100,
        Currency::Bai => 10000,
    }
}
