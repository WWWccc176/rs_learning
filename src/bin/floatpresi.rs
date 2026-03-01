fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    // 对于 f32，使用 f32::EPSILON
    // 计算差值的绝对值，判断是否小于极小值
    assert!((abc.0 + abc.1 - abc.2).abs() < f32::EPSILON);

    // 对于 f64，使用 f64::EPSILON
    assert!((xyz.0 + xyz.1 - xyz.2).abs() < f64::EPSILON);

    println!("浮点数比较通过！");
}

