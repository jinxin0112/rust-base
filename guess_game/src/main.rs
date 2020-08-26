use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("欢迎来到猜数字游戏!");

    let s = thread_rng().gen_range(1, 101);

    loop {
        println!("请输入你的数字: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("错误输入");
        let guess: u32 = match guess.trim().parse() {
            Ok(i) => i,
            Err(_) => continue,
        };

        match guess.cmp(&s) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            }
        }
    }
}
