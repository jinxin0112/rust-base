use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn draft() {
    println!("欢迎来到猜数字游戏");
    let secret_number = thread_rng().gen_range(1, 101);

    loop {
        let mut guess_number = String::new();
        io::stin().read_line(&mut guess_number).expect("输入错误！");
        let guess_number: u32 = guess_number.trim().parse().expect("转换错误！");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("太小！"),
            Ordering::Greater => println!("太大！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}
