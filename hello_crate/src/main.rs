mod a;

pub use crate::a::b::c;

mod foo {
    #[derive(Debug)]
    pub struct User {
        pub name: String,
        age: u8,
    }
    impl User {
        pub fn say_hello(user: User) {
            println!("hello, {:?}", user);
        }
        pub fn gen_user(name: &str) -> User {
            User {
                name: String::from(name),
                age: 26,
            }
        }
    }
}

fn main() {
    let mut king = foo::User::gen_user("jinxin");

    king.name = String::from("zhanghaixin");

    foo::User::say_hello(king);

    let res = c::hello_c();
    println!("res is {}", res);
}
