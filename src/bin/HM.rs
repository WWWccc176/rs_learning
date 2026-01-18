use std::collections::HashMap;

fn main() {
    let pencels = aaa();

    println!("{}", pencels.values().sum::<i32>());
}

fn aaa() -> HashMap<String, i32> {
    let mut pencel_case = HashMap::new();
    pencel_case.insert(String::from("pen"), 3);
    pencel_case.insert(String::from("pencel"), 1);
    pencel_case
}
