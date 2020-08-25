#![allow(unused)]
fn main() {
    #[derive(Debug)]
    enum Message {
        Success(String),
        // Error(u8, u8, u8),
        // Info { a: i32, b: i32 },
    }

    #[derive(Debug)]
    enum Bar {
        Jin(String),
        Li,
        Zhang,
    }

    enum Foo {
        A,
        B,
        C(Bar),
    }

    let a = Message::Success(String::from("jinxin"));

    println!("a is {:?}", &a);

    let r = test_match(Foo::C(Bar::Jin(String::from("xin"))));
    println!("r is {}", r);

    fn test_match(f: Foo) -> u8 {
        match f {
            Foo::A => 1,
            Foo::B => 2,
            Foo::C(state) => {
                println!("State Bar is {:?}!", state);
                25
            }
        }
    }

    fn test_if_let(f: Foo) {
        if let Foo::C(i) = f {
            println!("if let Bar is {:?}", i)
        } else {
            println!("hahahahah!")
        }
    }

    test_if_let(Foo::C(Bar::Zhang));

    let some_u8_value = Some(1u8);

    let b = b"Hello";

    let c = match some_u8_value {
        Some(i) => i,
        None => 0,
    };

    println!("some_u8_value is {:?}", some_u8_value);
    println!("b is {:?}", b);
    println!("c is {:?}", c);
}
