#![allow(unused)]
fn main() {
    use std::fs::{self, File};
    use std::io::{self, Read};
    // let v = vec![1, 2, 3];
    // println!("{}", v[99]);

    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("文件创建错误:{}", error),
    //         },
    //         other_error => panic!("文件读取错误:{:?}", other_error),
    //     },
    // };

    // let mut buffer = String::new();

    // f.read_to_string(&mut buffer).expect("文件读取错误");

    //     println!("file data {}", buffer);

    // fn read_file_string() -> Result<String, io::Error> {
    //     let mut f = File::open("hello.text")?;
    //     let mut buffer = String::new();
    //     f.read_to_string(&mut buffer)?;
    //     Ok(buffer)
    // }

    fn read_file_string() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(init: i32) -> Guess {
            if init < 1 || init > 100 {
                panic!("数字在 1 到 100之间")
            }

            Guess { value: init }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
