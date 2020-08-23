use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("欢迎来到 guess game!");

    let secret_number = thread_rng().gen_range(1, 101);
    // println!("随机数字是 {}", secret_number);

    loop {
        println!("请输入你数字:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read_line fail!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // 字符串转数字
        println!("你输入的数字是 {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
