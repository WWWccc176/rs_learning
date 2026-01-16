fn main() {
    const NUMBER: i32 = 3; //这里const在编译时编译器会将变量直接替换为3,于是，const可以写在外面也没有问题。
    //const必须显式指定数据类型
    println!("Number: {NUMBER}");
}
