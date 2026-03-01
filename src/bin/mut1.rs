fn main() {
    let x = Box::new(5);

    let mut y = x.clone();

    *y = 4; // 修改 y 管理的堆数据，不会影响到 x

    assert_eq!(*x, 5);
}
