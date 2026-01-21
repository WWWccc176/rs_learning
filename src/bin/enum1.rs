enum IpAddrKind {
    V4(String),
    V6(String),
}

impl IpAddrKind {
    fn change(&mut self, new_addr: String) {
        match self {
            // s 是内部 String 的可变引用
            IpAddrKind::V4(s) => {
                println!("正在将 V4 地址从 {} 修改为 {}", s, new_addr);
                *s = new_addr;
            }
            IpAddrKind::V6(s) => {
                println!("正在将 V6 地址从 {} 修改为 {}", s, new_addr);
                *s = new_addr;
            }
        }
    }

    // 辅助方法：打印当前地址
    fn display(&self) {
        match self {
            IpAddrKind::V4(addr) => println!("IPv4: {}", addr),
            IpAddrKind::V6(addr) => println!("IPv6: {}", addr),
        }
    }
}

fn main() {
    // 注意：要修改实例，必须声明为 mut
    let mut four = IpAddrKind::V4(String::from("1.1.1.2"));
    let six = IpAddrKind::V6(String::from("::1"));

    four.display();
    
    // 调用修改方法
    four.change(String::from("192.168.1.1"));
    
    four.display();
    six.display();
}