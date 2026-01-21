fn main() {
    let mut yu_dabao = User {
        email: String::from("ydb@123.com"),
        name: String::from("YDB"),
        active: true,
        accounts: 3,
    };

    yu_dabao.accounts = 5;

    let mut yu_dabao_ios = build_user(String::from("ydbios@123.com"), String::from("ydbios"));

    yu_dabao_ios.email = String::from("ydbios1@123.com");

    yu_dabao.name = String::from("YuXiangbao");
    yu_dabao.active = false;
}

struct User {
    email: String,
    name: String,
    active: bool,
    accounts: u8,
}

fn build_user(email: String, usermane: String) -> User {
    User {
        email,
        name: usermane,
        active: true,
        accounts: 1,
    }
}
